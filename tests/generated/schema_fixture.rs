#![allow(clippy::large_enum_variant)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::let_unit_value)]
#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};
mod tri_state_serde {
    use serde::{Deserialize, Deserializer};
    pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        T::deserialize(de).map(Some)
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fixture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    pub date: String,
    pub kind: FixtureKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<String>,
    #[serde(
        rename = "optionalNullable",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub optional_nullable: Option<Option<String>>,
    #[serde(rename = "requiredNullable")]
    pub required_nullable: Option<String>,
    pub union: FixtureUnion,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum FixtureUnion {
    String(String),
    Number(f64),
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum FixtureKind {
    #[default]
    #[serde(rename = "first")]
    First,
    #[serde(rename = "second")]
    Second,
}
impl FixtureKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::First => "first",
            Self::Second => "second",
        }
    }
}
impl ::std::fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl AsRef<str> for FixtureKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
