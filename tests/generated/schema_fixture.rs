#![allow(clippy::large_enum_variant)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::let_unit_value)]
#![allow(unreachable_patterns)]
#![allow(clippy::too_many_arguments)]
macro_rules! string_enum {
    ($(#[$meta:meta])* $name:ident { $($(#[$variant_meta:meta])* $variant:ident = $value:literal,)* }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $(#[doc = concat!("`", $value, "`")] $(#[$variant_meta])* $variant,)*
            /// A value this SDK version does not list, kept verbatim.
            Other(String),
        }
        impl $name {
            /// The value exactly as the API sends it.
            pub fn as_str(&self) -> &str {
                match self {
                    $(Self::$variant => $value,)*
                    Self::Other(value) => value,
                }
            }
        }
        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $($value => Self::$variant,)*
                    other => Self::Other(other.to_owned()),
                }
            }
        }
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }
        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                Ok(Self::from(String::deserialize(deserializer)?.as_str()))
            }
        }
    };
}
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
/// Fixture \[model\] with &lt;markup&gt;.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Fixture {
    /// The `amount` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The `date` field.
    pub date: String,
    /// The `kind` field.
    pub kind: FixtureKind,
    /// The `nestedValue` field.
    #[serde(rename = "nestedValue", skip_serializing_if = "Option::is_none")]
    pub nested_value: Option<FixtureNestedValue>,
    /// The `optional` field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<String>,
    /// The `optionalNullable` field.
    #[serde(
        rename = "optionalNullable",
        skip_serializing_if = "Option::is_none",
        default,
        deserialize_with = "tri_state_serde::deserialize"
    )]
    pub optional_nullable: Option<Option<String>>,
    /// Required, may be null.
    #[serde(rename = "requiredNullable")]
    pub required_nullable: Option<String>,
    /// The `union` field.
    pub union: FixtureUnion,
}
/// The `union` value.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum FixtureUnion {
    /// A string value.
    String(String),
    /// A number value.
    Number(f64),
}
/// The `nestedValue` value.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct FixtureNestedValue {
    /// The `innerName` field.
    #[serde(rename = "innerName", skip_serializing_if = "Option::is_none")]
    pub inner_name: Option<String>,
}
string_enum! {
    /// The `kind` value.
    FixtureKind {
        First = "first",
        Second = "second",
    }
}
