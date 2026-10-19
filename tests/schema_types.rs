#[rustfmt::skip]
#[allow(clippy::all)]
#[path = "generated/schema_fixture.rs"]
mod fixture;

use fixture::Fixture;
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
    let mut invalid = base();
    invalid["kind"] = json!("unknown");
    assert!(serde_json::from_value::<Fixture>(invalid).is_err());
}
