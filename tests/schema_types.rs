#[rustfmt::skip]
#[allow(clippy::all)]
#[path = "generated/schema_fixture.rs"]
mod fixture;

#[rustfmt::skip]
#[allow(clippy::all)]
#[path = "generated/enum_values.rs"]
mod enum_values;

use fixture::{Fixture, FixtureKind};
use roxyapi::{ApiError, ErrorCode};
use serde_json::{Value, json};

fn base() -> Value {
    json!({"requiredNullable":null,"union":2.5,"kind":"first","date":"1990-07-15"})
}

#[test]
fn required_nullable_serializes_null_and_accepts_missing_input() {
    let value: Fixture = serde_json::from_value(base()).unwrap();
    assert_eq!(value.required_nullable, None);
    assert_eq!(serde_json::to_value(&value).unwrap(), base());
    let mut missing = base();
    missing.as_object_mut().unwrap().remove("requiredNullable");
    let absent: Fixture = serde_json::from_value(missing).unwrap();
    assert_eq!(absent.required_nullable, None);
    assert_eq!(serde_json::to_value(absent).unwrap(), base());
}

#[test]
fn optional_and_nullable_fields_preserve_absent_null_and_value() {
    let absent: Fixture = serde_json::from_value(base()).unwrap();
    assert_eq!(absent.optional_nullable, None);
    let mut explicit = base();
    explicit["optionalNullable"] = Value::Null;
    let null: Fixture = serde_json::from_value(explicit.clone()).unwrap();
    assert_eq!(null.optional_nullable, Some(None));
    assert_eq!(serde_json::to_value(null).unwrap(), explicit);
    explicit["optionalNullable"] = json!("café");
    let text: Fixture = serde_json::from_value(explicit.clone()).unwrap();
    assert_eq!(text.optional_nullable, Some(Some("café".into())));
    assert_eq!(serde_json::to_value(text).unwrap(), explicit);
}

#[test]
fn unions_and_enums_preserve_wire_types() {
    for union in [json!(2.5), json!("Europe/London")] {
        let mut source = base();
        source["union"] = union;
        let parsed: Fixture = serde_json::from_value(source.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), source);
    }
    let mut invalid = base();
    invalid["union"] = json!({});
    assert!(serde_json::from_value::<Fixture>(invalid).is_err());
    let mut unknown = base();
    unknown["kind"] = json!("third");
    let parsed: Fixture = serde_json::from_value(unknown.clone()).unwrap();
    assert_eq!(parsed.kind, FixtureKind::Other("third".into()));
    assert_eq!(serde_json::to_value(parsed).unwrap(), unknown);
}

#[test]
fn nested_renamed_fields_round_trip() {
    let mut source = base();
    source["nestedValue"] = json!({"innerName":"Venus"});
    let parsed: Fixture = serde_json::from_value(source.clone()).unwrap();
    assert_eq!(
        parsed.nested_value.as_ref().unwrap().inner_name.as_deref(),
        Some("Venus")
    );
    assert_eq!(serde_json::to_value(parsed).unwrap(), source);
}

#[test]
fn every_enum_value_round_trips_and_unknown_values_are_kept() {
    let count = enum_values::every_enum_value();
    assert!(count > 0);
    eprintln!("Round-tripped {count} enum values");
}

#[test]
fn error_codes_match_the_specification() {
    let spec: Value = serde_json::from_str(include_str!("../specs/openapi.json")).unwrap();
    let codes = spec["info"]["x-error-codes"].as_array().unwrap();
    assert!(!codes.is_empty());
    for entry in codes {
        let wire = entry["code"].as_str().unwrap();
        let code = ErrorCode::from(wire);
        assert_ne!(code, ErrorCode::Other(wire.into()), "{wire}");
        assert_eq!(code.as_str(), wire);
        assert_eq!(code.to_string(), wire);
        assert_eq!(
            code.status(),
            entry["status"].as_u64().map(|status| status as u16)
        );
        assert_eq!(serde_json::to_value(&code).unwrap(), json!(wire));
        assert_eq!(
            serde_json::from_value::<ErrorCode>(json!(wire)).unwrap(),
            code
        );
    }
    let unknown = ErrorCode::from("future_code");
    assert_eq!(unknown, ErrorCode::Other("future_code".into()));
    assert_eq!(unknown.status(), None);
    let error: ApiError =
        serde_json::from_value(json!({"error":"Missing","code":"not_found"})).unwrap();
    assert_eq!(error.error_code(), ErrorCode::from("not_found"));
}

#[test]
fn error_tables_list_every_specification_code_as_a_typed_code() {
    let spec: Value = serde_json::from_str(include_str!("../specs/openapi.json")).unwrap();
    let expected: Vec<&str> = spec["info"]["x-error-codes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["code"].as_str().unwrap())
        .collect();
    for document in [include_str!("../README.md"), include_str!("../AGENTS.md")] {
        let start = document.find("<!-- BEGIN:ERRORS -->").unwrap();
        let end = document.find("<!-- END:ERRORS -->").unwrap();
        let listed: Vec<&str> = document[start..end]
            .lines()
            .filter_map(|row| {
                row.split('|')
                    .nth(2)?
                    .trim()
                    .strip_prefix('`')?
                    .strip_suffix('`')
            })
            .collect();
        assert_eq!(listed, expected);
        for code in listed {
            assert_ne!(
                ErrorCode::from(code),
                ErrorCode::Other(code.into()),
                "{code}"
            );
        }
    }
}
