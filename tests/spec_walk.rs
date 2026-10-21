#[allow(dead_code)]
mod common;

#[rustfmt::skip]
#[allow(clippy::all)]
#[path = "generated/dispatch.rs"]
mod dispatch;

use common::{Reply, Server};
use serde_json::{Map, Value, json};

fn resolved<'a>(schema: &'a Value, spec: &'a Value) -> &'a Value {
    match schema.get("$ref").and_then(Value::as_str) {
        Some(reference) => resolved(
            spec.pointer(reference.strip_prefix('#').unwrap()).unwrap(),
            spec,
        ),
        None => schema,
    }
}

/// Whether the schema or one of its union branches declares this JSON type.
fn declares(schema: &Value, kind: &str) -> bool {
    let direct = match schema.get("type") {
        Some(Value::String(declared)) => declared == kind,
        Some(Value::Array(declared)) => declared.iter().any(|declared| declared == kind),
        _ => false,
    };
    direct
        || ["anyOf", "oneOf"].iter().any(|key| {
            schema
                .get(*key)
                .and_then(Value::as_array)
                .is_some_and(|branches| branches.iter().any(|branch| declares(branch, kind)))
        })
}

/// A specification example for a scalar schema, as the declared JSON type.
fn example(schema: &Value) -> Option<Value> {
    let value = schema
        .get("example")
        .or_else(|| schema.get("examples")?.as_array()?.first())?;
    if value.is_object() || value.is_array() {
        return None;
    }
    let float = declares(schema, "number") && !declares(schema, "integer");
    Some(match value.as_f64() {
        Some(number) if float => json!(number),
        _ => value.clone(),
    })
}

/// A value for every declared property, optional ones included, preferring specification examples.
fn sample(schema: &Value, spec: &Value, depth: usize) -> Value {
    assert!(depth < 50, "recursive fixture schema");
    let schema = resolved(schema, spec);
    if let Some(value) = schema.get("const") {
        return value.clone();
    }
    if let Some(value) = example(schema) {
        return value;
    }
    if let Some(value) = schema
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|items| items.first())
    {
        return value.clone();
    }
    for key in ["oneOf", "anyOf"] {
        if let Some(variants) = schema.get(key).and_then(Value::as_array) {
            let variant = variants
                .iter()
                .find(|item| item.get("type").and_then(Value::as_str) != Some("null"))
                .unwrap();
            return sample(variant, spec, depth + 1);
        }
    }
    if let Some(parts) = schema.get("allOf").and_then(Value::as_array) {
        let mut result = Map::new();
        for part in parts {
            // An annotation-only part, such as a lone description, adds no fields.
            match sample(part, spec, depth + 1) {
                Value::Object(fields) => result.extend(fields),
                Value::Null => {}
                other => panic!("object intersection fixture: {other}"),
            }
        }
        return Value::Object(result);
    }
    let kind = schema.get("type").and_then(|value| {
        value.as_str().or_else(|| {
            value
                .as_array()?
                .iter()
                .filter_map(Value::as_str)
                .find(|kind| *kind != "null")
        })
    });
    match kind {
        Some("object") | None if schema.get("properties").is_some() => {
            let mut fields = Map::new();
            for (name, property) in schema["properties"].as_object().unwrap() {
                fields.insert(name.to_owned(), sample(property, spec, depth + 1));
            }
            Value::Object(fields)
        }
        Some("object") => match schema.get("additionalProperties") {
            Some(value) if value.as_object().is_some_and(|value| !value.is_empty()) => {
                json!({"sample": sample(value, spec, depth + 1)})
            }
            _ => json!({}),
        },
        Some("array") => {
            let count = schema
                .get("minItems")
                .and_then(Value::as_u64)
                .unwrap_or(1)
                .min(10);
            Value::Array(
                (0..count)
                    .map(|_| sample(&schema["items"], spec, depth + 1))
                    .collect(),
            )
        }
        Some("integer") => json!(schema.get("minimum").and_then(Value::as_i64).unwrap_or(1)),
        Some("number") => json!(
            schema
                .get("minimum")
                .and_then(Value::as_f64)
                .unwrap_or(1.25)
        ),
        Some("boolean") => json!(true),
        Some("string") => json!(match schema.get("format").and_then(Value::as_str) {
            Some("date") => "1990-01-15",
            Some("date-time") => "1990-01-15T14:30:00Z",
            Some("uuid") => "00000000-0000-4000-8000-000000000000",
            _ => "fixture",
        }),
        Some("null") | None => Value::Null,
        other => panic!("unsupported fixture type: {other:?}"),
    }
}

fn wire(value: &Value) -> String {
    match value {
        Value::String(value) => value.clone(),
        value => value.to_string(),
    }
}

#[tokio::test]
async fn every_spec_operation_sends_every_declared_field_and_decodes_every_response_field() {
    let spec: Value = serde_json::from_str(include_str!("../specs/openapi.json")).unwrap();
    let mut count = 0;
    for (path, item) in spec["paths"].as_object().unwrap() {
        for (method, operation) in item.as_object().unwrap() {
            if ![
                "get", "post", "put", "patch", "delete", "head", "options", "trace",
            ]
            .contains(&method.as_str())
            {
                continue;
            }
            let id = operation["operationId"].as_str().unwrap();
            let mut params = Map::new();
            let mut expected_queries = Vec::new();
            let mut expected_path = path.clone();
            for parameter in item
                .get("parameters")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .chain(
                    operation
                        .get("parameters")
                        .and_then(Value::as_array)
                        .into_iter()
                        .flatten(),
                )
            {
                let parameter = resolved(parameter, &spec);
                let name = parameter["name"].as_str().unwrap();
                let value =
                    example(parameter).unwrap_or_else(|| sample(&parameter["schema"], &spec, 0));
                match parameter["in"].as_str().unwrap() {
                    "path" => {
                        expected_path = expected_path.replace(&format!("{{{name}}}"), &wire(&value))
                    }
                    "query" => expected_queries.push((name.to_owned(), wire(&value))),
                    other => panic!("unsupported fixture parameter: {other}"),
                }
                params.insert(name.to_owned(), value);
            }
            let body = operation
                .get("requestBody")
                .map(|schema| {
                    sample(
                        &resolved(schema, &spec)["content"]["application/json"]["schema"],
                        &spec,
                        0,
                    )
                })
                .unwrap_or(Value::Null);
            let (status, response) = operation["responses"]
                .as_object()
                .unwrap()
                .iter()
                .filter_map(|(status, response)| {
                    status
                        .parse::<u16>()
                        .ok()
                        .filter(|value| (200..300).contains(value))
                        .map(|status| (status, resolved(response, &spec)))
                })
                .next()
                .unwrap();
            let response_body =
                sample(&response["content"]["application/json"]["schema"], &spec, 0);
            let server = Server::start(vec![Reply::new(status, response_body.to_string())]).await;
            let result =
                dispatch::call(&server.client(), id, Value::Object(params), body.clone()).await;
            assert!(result.is_ok(), "{id}: {result:?}; fixture {response_body}");
            assert_eq!(result.unwrap(), response_body, "{id}: response fields");
            let requests = server.observed();
            assert_eq!(requests.len(), 1, "{id}");
            let request = &requests[0];
            assert_eq!(request.method, method.to_ascii_uppercase(), "{id}");
            let url =
                roxyapi::reqwest::Url::parse(&format!("http://fixture{}", request.target)).unwrap();
            assert_eq!(url.path(), format!("/api/v2{expected_path}"), "{id}");
            let mut actual_queries: Vec<_> = url
                .query_pairs()
                .map(|(name, value)| (name.into_owned(), value.into_owned()))
                .collect();
            actual_queries.sort();
            expected_queries.sort();
            assert_eq!(actual_queries, expected_queries, "{id}");
            if body.is_null() {
                assert!(request.body.is_empty(), "{id}");
            } else {
                assert_eq!(
                    serde_json::from_slice::<Value>(&request.body).unwrap(),
                    body,
                    "{id}"
                );
            }
            count += 1;
        }
    }
    assert!(count > 0);
    eprintln!("Exercised {count} OpenAPI operations");
}
