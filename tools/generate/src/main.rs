use heck::{ToSnakeCase, ToUpperCamelCase};
use openapi_to_rust::{
    CodeGenerator, GeneratorConfig, SchemaAnalyzer,
    analysis::UntypedVerdict,
    type_mapping::{FloatPrecision, TypeMapper, TypeMappingConfig},
};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    fs,
    path::Path,
};
use syn::spanned::Spanned;
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn namespace(path: &str) -> Result<&str> {
    path.split('/')
        .find(|part| !part.is_empty())
        .ok_or_else(|| "empty operation path".into())
}
fn path_parameter(segment: &str) -> Result<Option<&str>> {
    if !segment.contains(['{', '}']) {
        return Ok(None);
    }
    let parameter = segment
        .strip_prefix('{')
        .and_then(|name| name.strip_suffix('}'))
        .filter(|name| !name.is_empty() && !name.contains(['{', '}']))
        .ok_or("only whole-segment path templates are supported")?;
    Ok(Some(parameter))
}
fn identifier(name: &str) -> String {
    let candidate = name.to_snake_case();
    if syn::parse_str::<syn::Ident>(&candidate).is_ok() {
        candidate
    } else {
        format!("r#{candidate}")
    }
}
fn write(root: &Path, file: &str, content: &str) -> Result<()> {
    let destination = root.join(file);
    fs::create_dir_all(destination.parent().ok_or("invalid output path")?)?;
    fs::write(destination, content)?;
    Ok(())
}
fn rust(code: &str) -> Result<String> {
    Ok(prettyplease::unparse(&syn::parse_file(code)?))
}
fn resolve<'a>(spec: &'a Value, mut value: &'a Value) -> Result<&'a Value> {
    let mut seen = BTreeSet::new();
    while let Some(reference) = value["$ref"].as_str() {
        if !seen.insert(reference) {
            return Err("cyclic reference".into());
        }
        let pointer = reference
            .strip_prefix('#')
            .ok_or("external parameter references are unsupported")?;
        value = spec.pointer(pointer).ok_or("unresolved local reference")?;
    }
    Ok(value)
}
fn scalar_schema(spec: &Value, schema: &Value) -> Result<()> {
    let schema = resolve(spec, schema)?;
    if schema["nullable"] == true {
        return Err("nullable parameters are unsupported".into());
    }
    if let Some(branches) = schema["anyOf"]
        .as_array()
        .or_else(|| schema["oneOf"].as_array())
    {
        for branch in branches {
            scalar_schema(spec, branch)?;
        }
        return Ok(());
    }
    let valid = |kind: &str| ["string", "integer", "number", "boolean"].contains(&kind);
    let supported = match &schema["type"] {
        Value::String(kind) => valid(kind),
        Value::Array(kinds) => kinds.iter().all(|kind| kind.as_str().is_some_and(valid)),
        _ => false,
    };
    if supported {
        Ok(())
    } else {
        Err("only scalar path and query schemas are supported".into())
    }
}
fn json_schema<'a>(spec: &'a Value, content: &'a Value) -> Result<&'a Value> {
    let content = content.as_object().ok_or("JSON content missing")?;
    if content.len() != 1 || !content.contains_key("application/json") {
        return Err("only application/json content is supported".into());
    }
    let schema = content["application/json"]
        .get("schema")
        .ok_or("JSON schema missing")?;
    resolve(spec, schema)
}
fn validate_json_contract(spec: &Value, operation: &Value) -> Result<()> {
    if let Some(body) = operation.get("requestBody") {
        let body = resolve(spec, body)?;
        json_schema(spec, &body["content"])?;
    }
    let responses = operation["responses"]
        .as_object()
        .ok_or("responses missing")?;
    let mut expected = None;
    for (status, response) in responses {
        if !status.starts_with('2') {
            continue;
        }
        let response = resolve(spec, response)?;
        let schema = json_schema(spec, &response["content"])?;
        if expected.is_some_and(|previous| previous != schema) {
            return Err("success response schemas differ".into());
        }
        expected = Some(schema);
    }
    expected.ok_or("JSON success response missing")?;
    Ok(())
}
fn operations(spec: &Value) -> Result<Vec<(String, String, Value)>> {
    let mut result = Vec::new();
    let mut ids = BTreeSet::new();
    let mut names = BTreeSet::new();
    let mut tags = BTreeMap::new();
    let declared_tags = spec["tags"].as_array().ok_or("tags missing")?;
    for (path, item) in spec["paths"].as_object().ok_or("paths missing")? {
        for segment in path.split('/').filter(|part| !part.is_empty()) {
            path_parameter(segment)?;
        }
        let item = resolve(spec, item)?;
        for (method, operation) in item.as_object().ok_or("invalid path item")? {
            if ![
                "get", "post", "put", "delete", "patch", "head", "options", "trace",
            ]
            .contains(&method.as_str())
            {
                continue;
            }
            validate_json_contract(spec, operation)?;
            let id = operation["operationId"]
                .as_str()
                .filter(|id| !id.is_empty())
                .ok_or("HTTP operation has no operationId")?;
            if !ids.insert(id) {
                return Err("duplicate operationId".into());
            }
            let group = namespace(path)?;
            if !names.insert((group, identifier(id))) {
                return Err("operation Rust names collide".into());
            }
            let operation_tags = operation["tags"]
                .as_array()
                .filter(|tags| !tags.is_empty())
                .ok_or("operation has no tag")?;
            for tag in operation_tags {
                let tag = tag.as_str().ok_or("invalid operation tag")?;
                if !declared_tags.iter().any(|declared| declared["name"] == tag) {
                    return Err("operation tag is undeclared".into());
                }
                if tags
                    .insert(tag, group)
                    .is_some_and(|previous| previous != group)
                {
                    return Err("one tag maps to multiple URL namespaces".into());
                }
            }
            let mut merged = BTreeMap::new();
            for source in [item, operation] {
                if let Some(parameters) = source["parameters"].as_array() {
                    for parameter in parameters {
                        let parameter = resolve(spec, parameter)?;
                        let name = parameter["name"].as_str().ok_or("parameter name missing")?;
                        let location = parameter["in"]
                            .as_str()
                            .ok_or("parameter location missing")?;
                        if !["path", "query"].contains(&location) {
                            return Err("unsupported parameter location".into());
                        }
                        if parameter["allowReserved"] == true {
                            return Err("allowReserved parameters are unsupported".into());
                        }
                        if let Some(style) = parameter["style"].as_str()
                            && ((location == "query" && style != "form")
                                || (location == "path" && style != "simple"))
                        {
                            return Err("unsupported parameter serialization style".into());
                        }
                        if location == "path" && parameter["required"] != true {
                            return Err("path parameters must be required".into());
                        }
                        scalar_schema(spec, &parameter["schema"])?;
                        merged.insert((location.to_string(), name.to_string()), parameter.clone());
                    }
                }
            }
            let mut operation = operation.clone();
            operation["parameters"] = Value::Array(merged.into_values().collect());
            result.push((path.clone(), method.to_uppercase(), operation));
        }
    }
    result.sort_by(|a, b| {
        a.2["operationId"]
            .as_str()
            .cmp(&b.2["operationId"].as_str())
    });
    Ok(result)
}
fn strip_docs(attributes: &mut Vec<syn::Attribute>) {
    attributes.retain(|attribute| !attribute.path().is_ident("doc"));
}
struct PublicDocs;
macro_rules! visit_docs {
    ($($method:ident, $kind:ident;)+) => {$ (fn $method(&mut self, node:&mut syn::$kind){strip_docs(&mut node.attrs); syn::visit_mut::$method(self,node);})+};
}
impl syn::visit_mut::VisitMut for PublicDocs {
    visit_docs! {
        visit_item_struct_mut, ItemStruct;
        visit_item_enum_mut, ItemEnum;
        visit_item_impl_mut, ItemImpl;
        visit_impl_item_fn_mut, ImplItemFn;
        visit_item_fn_mut, ItemFn;
        visit_field_mut, Field;
        visit_variant_mut, Variant;
        visit_item_mod_mut, ItemMod;
        visit_item_use_mut, ItemUse;
        visit_item_type_mut, ItemType;
        visit_item_const_mut, ItemConst;
    }
}
fn public_models(source: &str) -> Result<String> {
    use syn::visit_mut::VisitMut;
    let mut syntax = syn::parse_file(source)?;
    strip_docs(&mut syntax.attrs);
    PublicDocs.visit_file_mut(&mut syntax);
    Ok(prettyplease::unparse(&syntax))
}
fn marker(source: &str, name: &str, content: &str) -> Result<String> {
    let start = format!("<!-- BEGIN:{name} -->");
    let end = format!("<!-- END:{name} -->");
    let before = source.find(&start).ok_or("missing generated marker")? + start.len();
    let after = source[before..].find(&end).ok_or("missing marker end")? + before;
    Ok(format!(
        "{}\n{}\n{}",
        &source[..before],
        content.trim(),
        &source[after..]
    ))
}
fn common_calls(source: &str, groups: &[(String, String)]) -> Result<String> {
    let syntax = syn::parse_file(source)?;
    let original = |span: proc_macro2::Span| -> Result<&str> {
        source
            .get(span.byte_range())
            .ok_or_else(|| "invalid example source span".into())
    };
    let mut imports = Vec::new();
    let mut declarations = Vec::new();
    let mut functions = BTreeMap::new();
    for item in &syntax.items {
        match item {
            syn::Item::Use(item) => imports.push(original(item.span())?),
            syn::Item::Struct(item) => declarations.push(original(item.span())?),
            syn::Item::Fn(function) if function.sig.ident == "main" => {}
            syn::Item::Fn(function) => {
                if function.sig.asyncness.is_none() {
                    return Err("common-call functions must be async".into());
                }
                let name = function.sig.ident.to_string();
                if functions.insert(name, function).is_some() {
                    return Err("duplicate common-call namespace".into());
                }
            }
            _ => return Err("unsupported common-call example item".into()),
        }
    }
    let expected = groups.iter().map(|(name, _)| name).collect::<BTreeSet<_>>();
    if functions.keys().any(|name| !expected.contains(name)) {
        return Err("common-call function has no matching URL namespace".into());
    }
    let setup = groups
        .iter()
        .find(|(name, _)| name == "location" && functions.contains_key(name))
        .ok_or("location common-call setup missing")?;
    let mut rendered = Vec::new();
    for (name, label) in std::iter::once(setup).chain(
        groups
            .iter()
            .filter(|group| *group != setup && functions.contains_key(&group.0)),
    ) {
        let function = functions[name];
        let mut description = Vec::new();
        for attribute in &function.attrs {
            if let syn::Meta::NameValue(value) = &attribute.meta
                && attribute.path().is_ident("doc")
                && let syn::Expr::Lit(literal) = &value.value
                && let syn::Lit::Str(text) = &literal.lit
            {
                description.push(text.value().trim().to_string());
            } else {
                return Err("common-call functions support only documentation attributes".into());
            }
        }
        if description.is_empty() {
            return Err("common-call function description missing".into());
        }
        let start = function.sig.span().byte_range().start;
        let end = function.block.brace_token.span.close().byte_range().end;
        let code = source
            .get(start..end)
            .ok_or("invalid function source span")?;
        let mut snippet = imports
            .iter()
            .flat_map(|item| item.lines())
            .map(|line| format!("# {line}\n"))
            .collect::<String>();
        for declaration in &declarations {
            if &setup.0 == name {
                snippet.push_str(declaration);
                snippet.push_str("\n\n");
            } else {
                for line in declaration.lines() {
                    snippet.push_str(&format!("# {line}\n"));
                }
            }
        }
        snippet.push_str(code);
        rendered.push(format!(
            "### {label}\n\n{}\n\n```rust,no_run\n{snippet}\n```",
            description.join("\n")
        ));
    }
    Ok(rendered.join("\n\n"))
}
fn analyze_models(spec: Value) -> Result<(openapi_to_rust::SchemaAnalysis, GeneratorConfig)> {
    let types = TypeMappingConfig {
        float_precision: FloatPrecision::F64,
        ..TypeMappingConfig::conservative()
    };
    let mut analyzer = SchemaAnalyzer::with_type_mapper(spec, TypeMapper::new(types.clone()))?;
    let analysis = analyzer.analyze()?;
    for finding in analysis.untyped_fields() {
        if !matches!(finding.reason.verdict(), UntypedVerdict::Faithful) {
            return Err(format!("unsupported schema: {finding:?}").into());
        }
    }
    let config = GeneratorConfig {
        spec_path: "specs/openapi.json".into(),
        enable_async_client: false,
        enable_sse_client: false,
        types,
        ..GeneratorConfig::default()
    };
    Ok((analysis, config))
}
fn schema_fixture() -> Value {
    json!({"openapi":"3.1.0","info":{"title":"Schema fixture","version":"1"},"paths":{},"components":{"schemas":{"Fixture":{"type":"object","required":["requiredNullable","union","kind","date"],"properties":{
        "requiredNullable":{"type":["string","null"]},
        "optional":{"type":"string"},
        "optionalNullable":{"type":["string","null"]},
        "union":{"anyOf":[{"type":"string"},{"type":"number"}]},
        "kind":{"type":"string","enum":["first","second"]},
        "date":{"type":"string","format":"date"},
        "amount":{"type":"number","format":"float"}
    }}}}})
}
fn generate(spec: &Value, repository: &Path, output: &Path) -> Result<()> {
    if !spec["openapi"]
        .as_str()
        .is_some_and(|v| v.starts_with("3.1."))
    {
        return Err("OpenAPI 3.1 is required".into());
    }
    let ops = operations(spec)?;
    let mut model_spec = spec.clone();
    for (path, method, operation) in &ops {
        model_spec["paths"][path][method.to_lowercase()] = operation.clone();
    }
    let mut extra_schemas = serde_json::Map::new();
    for (path, method, operation) in &ops {
        let id = operation["operationId"]
            .as_str()
            .ok_or("missing operationId")?;
        let parameters = operation["parameters"].as_array();
        if let Some(parameters) = parameters.filter(|p| !p.is_empty()) {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();
            for parameter in parameters {
                let name = parameter["name"].as_str().ok_or("parameter name missing")?;
                if !["path", "query"].contains(
                    &parameter["in"]
                        .as_str()
                        .ok_or("parameter location missing")?,
                ) {
                    return Err("unsupported parameter location".into());
                }
                if !parameter["schema"].is_object() {
                    return Err("unsupported parameter reference".into());
                }
                if properties
                    .insert(name.into(), parameter["schema"].clone())
                    .is_some()
                {
                    return Err("parameter names collide".into());
                }
                if parameter["required"] == true {
                    required.push(name);
                }
            }
            extra_schemas.insert(
                format!("{}Params", id.to_upper_camel_case()),
                json!({"type":"object","properties":properties,"required":required}),
            );
        }
        let responses = model_spec["paths"][path][method.to_lowercase()]["responses"]
            .as_object_mut()
            .ok_or("responses missing")?;
        responses.retain(|status, _| status.starts_with('2'));
    }
    let schemas = model_spec["components"]["schemas"]
        .as_object_mut()
        .ok_or("schemas missing")?;
    for (name, schema) in extra_schemas {
        if schemas.insert(name, schema).is_some() {
            return Err("parameter model name collision".into());
        }
    }
    let (mut analysis, config) = analyze_models(model_spec)?;
    let generator = CodeGenerator::new(config);
    let mut models = generator.generate(&mut analysis)?;
    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut walker = String::from(
        "pub async fn call(client: &roxyapi::Roxy, operation_id: &str, params: serde_json::Value, body: serde_json::Value) -> Result<serde_json::Value,roxyapi::Error> { match operation_id {\n",
    );
    let mut signatures: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (path, method, operation) in &ops {
        let id = operation["operationId"]
            .as_str()
            .ok_or("missing operationId")?;
        let name = identifier(id);
        let group = namespace(path)?.to_string();
        let group_ident = identifier(&group);
        let operation_analysis = analysis
            .operations
            .get(id)
            .ok_or("operation missing from model analysis")?;
        let response = operation_analysis
            .response_schemas
            .get("200")
            .or_else(|| {
                operation_analysis
                    .response_schemas
                    .iter()
                    .find(|(code, _)| code.starts_with('2'))
                    .map(|(_, schema)| schema)
            })
            .ok_or("JSON success response missing")?;
        if !analysis.schemas.contains_key(response) {
            return Err("response model missing".into());
        }
        let parameters = operation["parameters"].as_array().filter(|p| !p.is_empty());
        let params_name = format!("{}Params", id.to_upper_camel_case());
        let body_name = operation_analysis
            .request_body
            .as_ref()
            .and_then(|body| body.schema_name());
        let mut arguments = Vec::new();
        let mut call_arguments = Vec::new();
        let mut decode = String::new();
        if parameters.is_some() {
            arguments.push(format!("params: &types::{params_name}"));
            decode.push_str(&format!("let params: roxyapi::types::{params_name} = serde_json::from_value(params).map_err(roxyapi::Error::Decode)?;\n"));
            call_arguments.push("&params");
        }
        if let Some(body_name) = body_name {
            arguments.push(format!("body: &types::{body_name}"));
            decode.push_str(&format!("let body: roxyapi::types::{body_name} = serde_json::from_value(body).map_err(roxyapi::Error::Decode)?;\n"));
            call_arguments.push("&body");
        } else if operation.get("requestBody").is_some() {
            return Err("unsupported request body".into());
        }
        let mut code = format!(
            "pub async fn {name}(&self{}{}) -> Result<types::{response}, crate::Error> {{\n",
            if arguments.is_empty() { "" } else { ", " },
            arguments.join(", ")
        );
        let mut segments = Vec::new();
        for part in path.split('/').filter(|p| !p.is_empty()) {
            if let Some(parameter) = path_parameter(part)? {
                let field = identifier(parameter);
                let temporary = identifier(&format!("path_{parameter}"));
                code.push_str(&format!("let {temporary} = params.{field}.to_string();\n"));
                segments.push(format!("&{temporary}"));
            } else {
                segments.push(format!("{part:?}"));
            }
        }
        code.push_str(&format!(
            "let request = self.0.request(reqwest::Method::{method}, &[{}])?;\n",
            segments.join(", ")
        ));
        if let Some(parameters) = parameters {
            for parameter in parameters {
                if parameter["in"] != "query" {
                    continue;
                }
                let wire = parameter["name"].as_str().ok_or("parameter name missing")?;
                let field = identifier(wire);
                if parameter["required"] == true {
                    code.push_str(&format!(
                        "let request = request.query(&[({wire:?}, &params.{field})]);\n"
                    ));
                } else {
                    code.push_str(&format!("let request = if let Some(value) = &params.{field} {{ request.query(&[({wire:?}, value)]) }} else {{ request }};\n"));
                }
            }
            let required = parameters
                .iter()
                .filter(|p| p["required"] == true)
                .collect::<Vec<_>>();
            if !required.is_empty()
                && required.iter().all(|p| {
                    p["schema"].get("enum").is_none()
                        && ["string", "number", "integer", "boolean"]
                            .contains(&p["schema"]["type"].as_str().unwrap_or(""))
                })
            {
                let fields = parameters
                    .iter()
                    .map(|p| {
                        Ok(format!(
                            "{}:Default::default()",
                            identifier(p["name"].as_str().ok_or("parameter name missing")?)
                        ))
                    })
                    .collect::<Result<Vec<_>>>()?;
                models.push_str(&format!(
                    "\nimpl Default for {params_name} {{ fn default()->Self {{Self{{{}}}}} }}\n",
                    fields.join(",")
                ));
            }
        }
        if body_name.is_some() {
            code.push_str("let request = request.json(body);\n");
        }
        code.push_str("self.0.send(request).await\n}\n");
        groups.entry(group.clone()).or_default().push(code);
        walker.push_str(&format!("{id:?}=>{{{decode} let result=client.{group_ident}().{name}({}).await?; serde_json::to_value(result).map_err(roxyapi::Error::Decode) }},\n",call_arguments.join(",")));
        signatures.entry(group).or_default().push(format!(
            "| `{id}` | `{}().{}({})` |",
            group_ident,
            name,
            call_arguments
                .iter()
                .map(|s| s.trim_start_matches('&'))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    walker.push_str("_=>Err(roxyapi::Error::Configuration(\"unknown operation\")),}}\n");
    let mut source = String::from("use super::types;\nuse crate::client::Core;\n");
    let mut domains = Vec::new();
    let mut method_rows = Vec::new();
    let tags = spec["tags"].as_array().ok_or("tags missing")?;
    let group_tag = |group: &str| -> Option<&str> {
        ops.iter()
            .find(|(path, _, _)| namespace(path).ok() == Some(group))
            .and_then(|(_, _, op)| op["tags"][0].as_str())
    };
    let mut ordered_groups = groups.iter().collect::<Vec<_>>();
    ordered_groups.sort_by_key(|(group, _)| {
        tags.iter()
            .position(|tag| tag["name"].as_str() == group_tag(group))
            .unwrap_or(usize::MAX)
    });
    let example_groups = ordered_groups
        .iter()
        .map(|(group, _)| {
            (
                identifier(group),
                group_tag(group).unwrap_or(group).to_string(),
            )
        })
        .collect::<Vec<_>>();
    let examples = common_calls(
        &fs::read_to_string(repository.join("examples/common_calls.rs"))?,
        &example_groups,
    )?;
    for (group, methods) in ordered_groups {
        method_rows.extend(
            signatures
                .remove(group)
                .ok_or("missing namespace signatures")?,
        );
        let class = group.to_upper_camel_case();
        let accessor = identifier(group);
        source.push_str(&format!("#[derive(Debug,Clone,Copy)] pub struct {class}<'a>(&'a Core); impl crate::Roxy {{pub fn {accessor}(&self)->{class}<'_>{{{class}(&self.core)}}}} impl {class}<'_>{{{}}}\n",methods.join("\n")));
        let label = group_tag(group).unwrap_or(group);
        domains.push(format!(
            "| [{label}](https://roxyapi.com/api-reference) | `{accessor}()` | {} |",
            methods.len()
        ));
    }
    let mut language_values = Vec::new();
    for (_, _, op) in &ops {
        if let Some(parameters) = op["parameters"].as_array() {
            for p in parameters {
                if p["name"] == "lang"
                    && let Some(values) = p["schema"]["enum"].as_array()
                {
                    if language_values.is_empty() {
                        language_values = values.clone();
                    } else if language_values != *values {
                        return Err("language parameter enums differ".into());
                    }
                }
            }
        }
    }
    let languages = language_values
        .iter()
        .map(|v| format!("`{}`", v.as_str().unwrap_or("")))
        .collect::<Vec<_>>()
        .join(", ");
    let domains = format!(
        "| Domain | Accessor | Operations |\n|---|---|---|\n{}",
        domains.join("\n")
    );
    let methods = format!(
        "| OpenAPI operation | Rust call |\n|---|---|\n{}",
        method_rows.join("\n")
    );
    for doc in ["README.md", "AGENTS.md"] {
        let original = fs::read_to_string(repository.join(doc))?;
        let mut document = marker(&original, "DOMAINS", &domains)?;
        document = marker(&document, "LANGS", &languages)?;
        document = marker(&document, "COMMON_CALLS", &examples)?;
        if doc == "AGENTS.md" {
            document = marker(&document, "METHODS", &methods)?;
        }
        if doc == "README.md" {
            let example = fs::read_to_string(repository.join("examples/quickstart.rs"))?;
            document = marker(
                &document,
                "QUICKSTART",
                &format!("```rust,no_run\n{example}```"),
            )?;
        }
        write(output, doc, &document)?;
    }
    write(
        output,
        "src/generated/mod.rs",
        "pub mod types;\nmod operations;\npub use operations::*;\n",
    )?;
    write(output, "src/generated/types.rs", &public_models(&models)?)?;
    write(output, "src/generated/operations.rs", &rust(&source)?)?;
    write(output, "tests/generated/dispatch.rs", &rust(&walker)?)?;
    let (mut fixture_analysis, fixture_config) = analyze_models(schema_fixture())?;
    let fixture = CodeGenerator::new(fixture_config).generate(&mut fixture_analysis)?;
    write(
        output,
        "tests/generated/schema_fixture.rs",
        &public_models(&fixture)?,
    )?;
    println!(
        "Generated {} operations in {} namespaces",
        ops.len(),
        groups.len()
    );
    Ok(())
}
fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 3 {
        return Err("usage: roxyapi-codegen SPEC REPOSITORY OUTPUT".into());
    }
    let spec: Value = serde_json::from_slice(&fs::read(&args[0])?)?;
    generate(&spec, Path::new(&args[1]), Path::new(&args[2]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn namespace_and_identifiers_follow_the_url_and_rust_syntax() {
        assert_eq!(
            namespace("/chinese-astrology/bazi").unwrap(),
            "chinese-astrology"
        );
        assert_eq!(identifier("generateNatalChart"), "generate_natal_chart");
        assert_eq!(identifier("type"), "r#type");
        assert!(namespace("/").is_err());
    }
    #[test]
    fn path_templates_require_one_complete_segment() {
        assert_eq!(path_parameter("literal.json").unwrap(), None);
        assert_eq!(path_parameter("{id}").unwrap(), Some("id"));
        for segment in ["{id}.json", "prefix{id}", "{id}{other}", "{}", "{id", "id}"] {
            assert!(path_parameter(segment).is_err(), "{segment}");
        }
        let mut spec = operation_fixture();
        let operation = spec["paths"]
            .as_object_mut()
            .unwrap()
            .remove("/fixture")
            .unwrap();
        spec["paths"]["/fixture/{id}.json"] = operation;
        let error = generate(&spec, Path::new("."), Path::new("unused")).unwrap_err();
        assert!(error.to_string().contains("whole-segment path templates"));
    }
    #[test]
    fn common_calls_preserve_source_and_share_visible_setup() {
        let source = r#"use roxyapi::{Roxy, types};
struct Birth { city: String }
/// Read a chart for München.
async fn astrology(roxy: &Roxy, birth: &Birth) {
    let nested = || { "{literal}" };
    println!("{}", nested());
}
/// Resolve a birthplace once.
async fn location(roxy: &Roxy) -> Birth {
    Birth { city: "München".into() }
}
fn main() {}
"#;
        let groups = vec![
            ("astrology".into(), "Western Astrology".into()),
            ("location".into(), "Location".into()),
        ];
        let docs = common_calls(source, &groups).unwrap();
        assert!(docs.starts_with("### Location"));
        assert!(docs.contains("\nstruct Birth { city: String }\n"));
        assert!(docs.contains("\n# struct Birth { city: String }\n"));
        assert!(docs.contains("    let nested = || { \"{literal}\" };"));
        assert!(docs.contains("München"));
        assert!(!docs.contains("fn main"));
        let mut expanded = groups;
        expanded.push(("future".into(), "Future Domain".into()));
        assert_eq!(common_calls(source, &expanded).unwrap(), docs);
    }
    #[test]
    fn common_calls_reject_invalid_namespaces_or_missing_setup() {
        let groups = vec![("astrology".into(), "Western Astrology".into())];
        for source in [
            "fn main() {}",
            "/// Extra\nasync fn other() {}",
            "/// First\nasync fn astrology() {}\n/// Duplicate\nasync fn astrology() {}",
            "/// Synchronous\nfn astrology() {}",
        ] {
            assert!(common_calls(source, &groups).is_err());
        }
    }
    #[test]
    fn scalar_configuration_and_nullable_union_shapes_are_preserved() {
        let (mut analysis, config) = analyze_models(schema_fixture()).unwrap();
        let source = CodeGenerator::new(config).generate(&mut analysis).unwrap();
        assert!(!source.contains("chrono::"));
        assert!(source.contains("pub date: String"));
        assert!(source.contains("pub amount: Option<f64>"));
        assert!(source.contains("Option<Option<String>>"));
        assert!(source.contains("String(String)"));
        assert!(source.contains("Number(f64)"));
    }
    #[test]
    fn malformed_specs_fail_before_writing() {
        assert!(
            generate(
                &json!({"openapi":"3.0.3"}),
                Path::new("."),
                Path::new("unused")
            )
            .is_err()
        );
        assert!(
            generate(
                &json!({"openapi":"3.1.0","paths":{}}),
                Path::new("."),
                Path::new("unused")
            )
            .is_err()
        );
    }
    fn operation_fixture() -> Value {
        json!({"openapi":"3.1.0","info":{"title":"Fixture","version":"1"},"tags":[{"name":"Fixtures"}],"paths":{"/fixture":{"get":{"operationId":"getFixture","tags":["Fixtures"],"responses":{"200":{"description":"Fixture","content":{"application/json":{"schema":{"type":"string"}}}}}}}},"components":{"schemas":{}}})
    }
    #[test]
    fn request_media_types_are_checked_before_emission() {
        let mut spec = operation_fixture();
        spec["paths"]["/fixture"]["get"]["requestBody"] = json!({
            "content":{"application/json":{"schema":{"type":"object"}}}
        });
        assert!(operations(&spec).is_ok());
        for media in [
            "multipart/form-data",
            "application/x-www-form-urlencoded",
            "text/plain",
        ] {
            spec["paths"]["/fixture"]["get"]["requestBody"]["content"] = json!({
                media:{"schema":{"type":"object"}}
            });
            let error = generate(&spec, Path::new("."), Path::new("unused")).unwrap_err();
            assert!(error.to_string().contains("application/json"));
        }
        spec["paths"]["/fixture"]["get"]["requestBody"]["content"] = json!({
            "application/json":{"schema":{"type":"object"}},
            "multipart/form-data":{"schema":{"type":"object"}}
        });
        assert!(operations(&spec).is_err());
    }
    #[test]
    fn success_statuses_require_the_same_json_schema() {
        let mut spec = operation_fixture();
        let same = spec["paths"]["/fixture"]["get"]["responses"]["200"].clone();
        spec["paths"]["/fixture"]["get"]["responses"]["201"] = same;
        assert!(operations(&spec).is_ok());
        spec["paths"]["/fixture"]["get"]["responses"]["201"]["content"]["application/json"]["schema"] =
            json!({"type":"number"});
        let error = generate(&spec, Path::new("."), Path::new("unused")).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("success response schemas differ")
        );
        spec["paths"]["/fixture"]["get"]["responses"]["201"] = json!({"description":"No content"});
        assert!(operations(&spec).is_err());
    }
    #[test]
    fn identical_referenced_success_schemas_are_supported() {
        let mut spec = operation_fixture();
        spec["components"]["schemas"]["Text"] = json!({"type":"string"});
        spec["components"]["responses"]["Text"] = json!({"description":"Text","content":{"application/json":{"schema":{"$ref":"#/components/schemas/Text"}}}});
        spec["paths"]["/fixture"]["get"]["responses"]["200"] =
            json!({"$ref":"#/components/responses/Text"});
        spec["paths"]["/fixture"]["get"]["responses"]["201"] = json!({"description":"Created","content":{"application/json":{"schema":{"type":"string"}}}});
        assert!(operations(&spec).is_ok());
        spec["components"]["schemas"]["Text"] = json!({"type":"number"});
        assert!(operations(&spec).is_err());
    }
    #[test]
    fn every_operation_needs_a_unique_id_and_rust_name() {
        let mut missing = operation_fixture();
        missing["paths"]["/fixture"]["get"]
            .as_object_mut()
            .unwrap()
            .remove("operationId");
        assert!(operations(&missing).is_err());
        let mut duplicate = operation_fixture();
        duplicate["paths"]["/fixture"]["post"] = duplicate["paths"]["/fixture"]["get"].clone();
        assert!(operations(&duplicate).is_err());
        duplicate["paths"]["/fixture"]["post"]["operationId"] = json!("get_fixture");
        assert!(operations(&duplicate).is_err());
    }
    #[test]
    fn tags_cannot_be_undeclared_or_span_namespaces() {
        let mut spec = operation_fixture();
        spec["paths"]["/other"]["get"] = spec["paths"]["/fixture"]["get"].clone();
        spec["paths"]["/other"]["get"]["operationId"] = json!("getOther");
        assert!(operations(&spec).is_err());
        let mut undeclared = operation_fixture();
        undeclared["paths"]["/fixture"]["get"]["tags"] = json!(["Missing"]);
        assert!(operations(&undeclared).is_err());
    }
    #[test]
    fn inherited_parameter_references_merge_with_operation_overrides() {
        let mut spec = operation_fixture();
        spec["components"]["parameters"]["Term"] =
            json!({"name":"q","in":"query","schema":{"type":"string"}});
        spec["paths"]["/fixture"]["parameters"] = json!([{"$ref":"#/components/parameters/Term"}]);
        spec["paths"]["/fixture"]["get"]["parameters"] =
            json!([{"name":"q","in":"query","required":true,"schema":{"type":"integer"}}]);
        let result = operations(&spec).unwrap();
        assert_eq!(result[0].2["parameters"].as_array().unwrap().len(), 1);
        assert_eq!(result[0].2["parameters"][0]["schema"]["type"], "integer");
        assert_eq!(result[0].2["parameters"][0]["required"], true);
        spec["paths"]["/fixture"]["get"]["parameters"][0]["schema"] =
            json!({"type":"array","items":{"type":"string"}});
        assert!(operations(&spec).is_err());
        spec["paths"]["/fixture"]["get"]["parameters"][0]["schema"] = json!({"type":"string"});
        spec["paths"]["/fixture"]["get"]["parameters"][0]["style"] = json!("deepObject");
        assert!(operations(&spec).is_err());
    }
    #[test]
    fn nullable_parameters_fail_before_request_generation() {
        let mut spec = operation_fixture();
        for schema in [
            json!({"type":"null"}),
            json!({"type":["string","null"]}),
            json!({"anyOf":[{"type":"number"},{"type":"null"}]}),
            json!({"type":"string","nullable":true}),
        ] {
            spec["paths"]["/fixture"]["get"]["parameters"] = json!([{
                "name":"q","in":"query","schema":schema
            }]);
            assert!(operations(&spec).is_err());
        }
        spec["paths"]["/fixture"]["get"]["parameters"][0]["schema"] =
            json!({"anyOf":[{"type":"number"},{"type":"string"}]});
        assert!(operations(&spec).is_ok());
    }
    #[test]
    fn unresolved_schema_references_fail_closed() {
        let mut spec = operation_fixture();
        spec["paths"]["/fixture"]["get"]["responses"]["200"]["content"]["application/json"]["schema"] =
            json!({"$ref":"#/components/schemas/Missing"});
        assert!(generate(&spec, Path::new("."), Path::new("unused")).is_err());
    }
    #[test]
    fn doc_metadata_is_removed_without_changing_a_description_field() {
        let source = "/// Metadata\n#[derive(Debug)] pub struct Result { /// Display text\n pub description: String }";
        let output = public_models(source).unwrap();
        assert!(output.contains("pub description: String"));
        assert!(!output.contains("Metadata"));
        assert!(!output.contains("Display text"));
    }
}
