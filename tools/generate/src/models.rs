//! Public model emission on top of the generator output.
//!
//! Every analyzed schema is tagged before generation so each emitted struct and enum names the
//! schema it came from. This pass then replaces generator prose with escaped specification
//! descriptions, marks every model `#[non_exhaustive]`, renders every string enum through one
//! macro that keeps unknown values in `Other(String)`, gives every type a caller builds a
//! constructor with setters, and keeps `Default` only on those request types, so an additive
//! specification change stays an additive Rust change.
use crate::Result;
use openapi_to_rust::analysis::{SchemaAnalysis, SchemaType};
use std::collections::{BTreeMap, BTreeSet};
use syn::visit::Visit;

const TAG: &str = "roxyapi-schema:";
const OTHER: &str = "Other";

/// One definition for every string enum: known variants plus `Other`, lossless in both directions.
pub const STRING_ENUM: &str = r#"macro_rules! string_enum {
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
"#;

/// Replace each schema description with its tag and return the original descriptions.
pub fn tag_schemas(analysis: &mut SchemaAnalysis) -> BTreeMap<String, Option<String>> {
    analysis
        .schemas
        .iter_mut()
        .map(|(name, schema)| {
            let original = schema.description.replace(format!("{TAG}{name}"));
            (name.clone(), original)
        })
        .collect()
}

/// One public struct field as emitted.
pub struct Field {
    pub ident: syn::Ident,
    pub wire: String,
    pub optional: bool,
    pub flatten: bool,
    pub ty: syn::Type,
}

/// The emitted models plus what the rest of generation reads from them.
pub struct Models {
    pub source: String,
    pub structs: BTreeMap<String, Vec<Field>>,
    /// Rust enum name and known wire values of every string enum, in specification order.
    pub enums: Vec<(String, Vec<String>)>,
}

fn doc_tag(attributes: &[syn::Attribute]) -> Option<String> {
    attributes
        .iter()
        .find_map(|attribute| match &attribute.meta {
            syn::Meta::NameValue(value) if attribute.path().is_ident("doc") => match &value.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(text),
                    ..
                }) => text.value().strip_prefix(TAG).map(str::to_string),
                _ => None,
            },
            _ => None,
        })
}

fn strip_docs(attributes: &mut Vec<syn::Attribute>) {
    attributes.retain(|attribute| !attribute.path().is_ident("doc"));
}

/// Escape one specification text for rustdoc: no intra-doc links, HTML, doctests or bare URLs.
fn doc_lines(text: &str) -> Vec<String> {
    let mut fenced = false;
    text.trim()
        .lines()
        .map(|line| {
            let line = line.trim_start();
            if let Some(language) = line.strip_prefix("```") {
                fenced = !fenced;
                return if fenced && language.trim().is_empty() {
                    "```text".to_string()
                } else {
                    line.to_string()
                };
            }
            if fenced {
                return line.to_string();
            }
            let mut result = String::new();
            if line.starts_with('#') {
                result.push('\\');
            }
            let balanced = line.matches('`').count() % 2 == 0;
            for (index, part) in line.split('`').enumerate() {
                if index > 0 {
                    result.push_str(if balanced { "`" } else { "\\`" });
                }
                if balanced && index % 2 == 1 {
                    result.push_str(part);
                } else {
                    result.push_str(&escape_prose(part));
                }
            }
            result
        })
        .collect()
}

fn escape_prose(text: &str) -> String {
    let mut result = String::new();
    let mut rest = text;
    while let Some(start) = ["http://", "https://"]
        .iter()
        .filter_map(|scheme| rest.find(scheme))
        .min()
    {
        result.push_str(&escape_markup(&rest[..start]));
        let length = rest[start..]
            .find(char::is_whitespace)
            .unwrap_or(rest.len() - start);
        let url = rest[start..start + length].trim_end_matches(['.', ',', ';', ':', ')', '"']);
        result.push_str(&format!("<{url}>"));
        rest = &rest[start + url.len()..];
    }
    result.push_str(&escape_markup(rest));
    result
}

fn escape_markup(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Doc attributes for a paragraph list; empty strings separate paragraphs.
fn paragraph_lines(paragraphs: &[String]) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in paragraphs.iter().filter(|text| !text.trim().is_empty()) {
        if !lines.is_empty() {
            lines.push(String::new());
        }
        lines.extend(
            doc_lines(paragraph)
                .into_iter()
                .map(|line| format!(" {line}")),
        );
    }
    lines
}

fn doc_attributes(paragraphs: &[String]) -> Vec<syn::Attribute> {
    paragraph_lines(paragraphs)
        .into_iter()
        .map(|line| syn::parse_quote!(#[doc = #line]))
        .collect()
}

/// `///` lines for source assembled as text.
pub fn doc_comment(paragraphs: &[String]) -> String {
    paragraph_lines(paragraphs)
        .iter()
        .map(|line| format!("///{line}\n"))
        .collect()
}

/// One `string_enum!` invocation laid out as source: `variants` holds doc comment, identifier and wire value.
pub fn string_enum_source(docs: &str, name: &str, variants: &[(String, String, String)]) -> String {
    let indent = |text: &str, depth: &str| -> String {
        text.lines()
            .map(|line| format!("{depth}{line}\n"))
            .collect()
    };
    let mut source = format!("string_enum! {{\n{}    {name} {{\n", indent(docs, "    "));
    for (docs, ident, value) in variants {
        source.push_str(&indent(docs, "        "));
        source.push_str(&format!("        {ident} = {value:?},\n"));
    }
    source.push_str("    }\n}\n");
    source
}

fn serde_field(attributes: &[syn::Attribute]) -> Result<(Option<String>, bool, bool)> {
    let (mut rename, mut optional, mut flatten) = (None, false, false);
    for attribute in attributes.iter().filter(|a| a.path().is_ident("serde")) {
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("flatten") {
                flatten = true;
            }
            if meta.input.peek(syn::Token![=]) {
                let value: syn::LitStr = meta.value()?.parse()?;
                if meta.path.is_ident("rename") {
                    rename = Some(value.value());
                } else if meta.path.is_ident("skip_serializing_if") {
                    optional = true;
                }
            }
            Ok(())
        })?;
    }
    Ok((rename, optional, flatten))
}

#[derive(Default)]
struct Names(BTreeSet<String>);
impl<'a> Visit<'a> for Names {
    fn visit_path(&mut self, path: &'a syn::Path) {
        if let Some(segment) = path.segments.last() {
            self.0.insert(segment.ident.to_string());
        }
        syn::visit::visit_path(self, path);
    }
}

fn without_default(attribute: &syn::Attribute) -> Result<syn::Attribute> {
    if !attribute.path().is_ident("derive") {
        return Ok(attribute.clone());
    }
    let mut kept = Vec::new();
    attribute.parse_nested_meta(|meta| {
        if !meta.path.is_ident("Default") {
            kept.push(meta.path);
        }
        Ok(())
    })?;
    Ok(syn::parse_quote!(#[derive(#(#kept),*)]))
}

/// Replace generator docs with the specification text, optionally closing the item to exhaustive use.
fn document(attributes: &mut Vec<syn::Attribute>, text: &str, non_exhaustive: bool) {
    strip_docs(attributes);
    let mut documented = doc_attributes(&[text.to_string()]);
    documented.append(attributes);
    if non_exhaustive {
        documented.push(syn::parse_quote!(#[non_exhaustive]));
    }
    *attributes = documented;
}

fn unwrap_option(ty: &syn::Type) -> Option<&syn::Type> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    match &segment.arguments {
        syn::PathArguments::AngleBracketed(arguments) => match arguments.args.first()? {
            syn::GenericArgument::Type(inner) => Some(inner),
            _ => None,
        },
        _ => None,
    }
}

/// The invocation for one generated string enum, after checking each variant against its wire value.
fn string_enum(item: &syn::ItemEnum, docs: &str, values: &[String]) -> Result<String> {
    let name = &item.ident;
    if item.variants.len() != values.len() {
        return Err(format!("string enum {name} lost a value").into());
    }
    let mut variants = Vec::new();
    for (variant, value) in item.variants.iter().zip(values) {
        let ident = &variant.ident;
        let (rename, _, _) = serde_field(&variant.attrs)?;
        if rename.unwrap_or_else(|| ident.to_string()) != *value || ident == OTHER {
            return Err(format!("string enum {name} cannot represent {value:?}").into());
        }
        variants.push((String::new(), ident.to_string(), value.clone()));
    }
    Ok(string_enum_source(docs, &name.to_string(), &variants))
}

fn setter_name(ident: &syn::Ident) -> syn::Ident {
    let name = ident.to_string();
    syn::Ident::new(
        &format!("with_{}", name.strip_prefix("r#").unwrap_or(&name)),
        proc_macro2::Span::call_site(),
    )
}

fn required_order(analysis: &SchemaAnalysis, schema: &str, fields: &[Field]) -> Vec<usize> {
    let declared = analysis.schemas[schema].original["required"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut order = declared
        .iter()
        .filter_map(|wire| {
            fields
                .iter()
                .position(|field| !field.optional && !field.flatten && field.wire == *wire)
        })
        .collect::<Vec<_>>();
    for (index, field) in fields.iter().enumerate() {
        if !field.optional && !field.flatten && !order.contains(&index) {
            order.push(index);
        }
    }
    order
}

fn constructor(
    analysis: &SchemaAnalysis,
    schema: &str,
    name: &syn::Ident,
    fields: &[Field],
) -> syn::Item {
    let required = required_order(analysis, schema, fields);
    let parameters = required.iter().map(|&index| {
        let (ident, ty) = (&fields[index].ident, &fields[index].ty);
        quote::quote!(#ident: #ty)
    });
    let initializers = fields.iter().enumerate().map(|(index, field)| {
        let ident = &field.ident;
        if required.contains(&index) {
            quote::quote!(#ident)
        } else {
            quote::quote!(#ident: Default::default())
        }
    });
    let setters = fields
        .iter()
        .enumerate()
        .filter(|(index, _)| !required.contains(index))
        .map(|(_, field)| {
            let (ident, setter) = (&field.ident, setter_name(&field.ident));
            let (ty, value, text) = match unwrap_option(&field.ty).filter(|_| !field.flatten) {
                Some(inner) if unwrap_option(inner).is_some() => (
                    inner,
                    quote::quote!(Some(value)),
                    format!(" Sets `{}`. `None` sends an explicit null.", field.wire),
                ),
                Some(inner) => (
                    inner,
                    quote::quote!(Some(value)),
                    format!(" Sets `{}`.", field.wire),
                ),
                None => (
                    &field.ty,
                    quote::quote!(value),
                    format!(" Sets `{}`.", field.wire),
                ),
            };
            quote::quote! {
                #[doc = #text]
                #[must_use]
                pub fn #setter(mut self, value: #ty) -> Self {
                    self.#ident = #value;
                    self
                }
            }
        });
    syn::parse_quote! {
        impl #name {
            /// Builds the value from its required fields, in specification order. Add optional fields with the `with_` setters.
            pub fn new(#(#parameters),*) -> Self {
                Self { #(#initializers),* }
            }
            #(#setters)*
        }
    }
}

/// Every model is documented: its own description, else the operation it serves, else the field that holds it.
fn model_docs(
    analysis: &SchemaAnalysis,
    descriptions: &BTreeMap<String, Option<String>>,
    schemas: &BTreeMap<String, String>,
    references: &BTreeMap<String, BTreeSet<String>>,
    structs: &BTreeMap<String, Vec<Field>>,
    property_doc: impl Fn(&str, &str) -> Option<String>,
) -> BTreeMap<String, String> {
    let rust_name = |schema: &str| {
        schemas
            .iter()
            .find(|(_, s)| *s == schema)
            .map(|(n, _)| n.clone())
    };
    let mut roles = BTreeMap::new();
    for operation in analysis.operations.values() {
        let id = &operation.operation_id;
        for schema in operation.response_schemas.values() {
            if let Some(name) = rust_name(schema) {
                roles
                    .entry(name)
                    .or_insert(format!("Response body of `{id}`."));
            }
        }
        if let Some(name) = operation
            .request_body
            .as_ref()
            .and_then(|body| body.schema_name())
            .and_then(rust_name)
        {
            roles
                .entry(name)
                .or_insert(format!("Request body of `{id}`."));
        }
    }
    let mut holders = BTreeMap::new();
    for (owner, fields) in structs {
        for field in fields {
            let mut names = Names::default();
            names.visit_type(&field.ty);
            let text = if names.0.contains("Vec") {
                format!("One entry of `{}`.", field.wire)
            } else {
                property_doc(&schemas[owner], &field.wire)
                    .unwrap_or_else(|| format!("The `{}` value.", field.wire))
            };
            for name in names
                .0
                .iter()
                .filter(|name| schemas.contains_key(*name) && *name != owner)
            {
                holders.entry(name.clone()).or_insert_with(|| text.clone());
            }
        }
    }
    for (owner, names) in references {
        if !structs.contains_key(owner) {
            for name in names
                .iter()
                .filter(|name| schemas.contains_key(*name) && *name != owner)
            {
                holders
                    .entry(name.clone())
                    .or_insert(format!("One form of [`{owner}`]."));
            }
        }
    }
    schemas
        .iter()
        .map(|(name, schema)| {
            let text = descriptions[schema]
                .clone()
                .or_else(|| roles.get(name).cloned())
                .or_else(|| holders.get(name).cloned())
                .unwrap_or_else(|| format!("The `{schema}` model."));
            (name.clone(), text)
        })
        .collect()
}

/// A union variant is documented by the form of its value.
fn variant_doc(variant: &syn::Variant, schemas: &BTreeMap<String, String>) -> String {
    let mut names = Names::default();
    names.visit_fields(&variant.fields);
    match names.0.iter().find(|name| schemas.contains_key(*name)) {
        Some(name) => format!("A [`{name}`] value."),
        None if names.0.contains("String") => "A string value.".into(),
        None if names.0.contains("f64") => "A number value.".into(),
        None if names.0.contains("i64") => "An integer value.".into(),
        None if names.0.contains("bool") => "A boolean value.".into(),
        None => format!("The `{}` form.", variant.ident),
    }
}

/// Apply documentation, compatibility attributes and constructors to generated models.
///
/// `roots` names the schemas callers construct: request bodies and parameter sets. Every model
/// reachable from them through field and variant types is constructed by callers too.
pub fn emit(
    source: &str,
    analysis: &SchemaAnalysis,
    descriptions: &BTreeMap<String, Option<String>>,
    roots: &BTreeSet<String>,
) -> Result<Models> {
    let mut syntax = syn::parse_file(source)?;
    strip_docs(&mut syntax.attrs);
    let mut schemas = BTreeMap::new();
    let mut references: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut structs = BTreeMap::new();
    for item in &syntax.items {
        let (name, attributes) = match item {
            syn::Item::Struct(item) => (item.ident.to_string(), &item.attrs),
            syn::Item::Enum(item) => (item.ident.to_string(), &item.attrs),
            syn::Item::Type(item) => (item.ident.to_string(), &item.attrs),
            _ => continue,
        };
        let Some(schema) = doc_tag(attributes) else {
            if matches!(item, syn::Item::Type(_)) {
                continue;
            }
            return Err(format!("generated model {name} has no source schema").into());
        };
        let mut names = Names::default();
        names.visit_item(item);
        references.insert(name.clone(), names.0);
        if let syn::Item::Struct(item) = item {
            let mut fields = Vec::new();
            for field in &item.fields {
                let ident = field.ident.clone().ok_or("tuple model")?;
                let (rename, optional, flatten) = serde_field(&field.attrs)?;
                let plain = ident.to_string();
                fields.push(Field {
                    wire: rename.unwrap_or_else(|| plain.trim_start_matches("r#").to_string()),
                    ident,
                    optional,
                    flatten,
                    ty: field.ty.clone(),
                });
            }
            structs.insert(name.clone(), fields);
        }
        schemas.insert(name, schema);
    }
    let rust_name = |schema: &str| schemas.iter().find(|(_, s)| *s == schema).map(|(n, _)| n);
    let mut constructed = BTreeSet::new();
    let mut pending = roots
        .iter()
        .map(|root| rust_name(root).cloned().ok_or("unknown request model"))
        .collect::<std::result::Result<Vec<_>, _>>()?;
    while let Some(name) = pending.pop() {
        if constructed.insert(name.clone()) {
            pending.extend(
                references[&name]
                    .iter()
                    .filter(|other| schemas.contains_key(*other) && **other != name)
                    .cloned(),
            );
        }
    }
    let property_doc = |schema: &str, wire: &str| match &analysis.schemas[schema].schema_type {
        SchemaType::Object { properties, .. } => properties
            .get(wire)
            .and_then(|property| property.description.clone()),
        _ => None,
    };
    let model_docs = model_docs(
        analysis,
        descriptions,
        &schemas,
        &references,
        &structs,
        |owner, wire| property_doc(owner, wire),
    );
    let field_doc = |owner: &str, field: &Field| {
        if field.flatten {
            return "Every other key of the object, by name.".to_string();
        }
        property_doc(&schemas[owner], &field.wire).unwrap_or_else(|| {
            let mut names = Names::default();
            names.visit_type(&field.ty);
            names
                .0
                .iter()
                .find_map(|name| descriptions.get(schemas.get(name)?)?.clone())
                .unwrap_or_else(|| format!("The `{}` field.", field.wire))
        })
    };
    let mut constructors = Vec::new();
    let mut enums = Vec::new();
    let mut string_enums = BTreeSet::new();
    let mut rendered = BTreeMap::new();
    for item in &mut syntax.items {
        let mut replacement = None;
        match item {
            syn::Item::Struct(item) => {
                let name = item.ident.to_string();
                let schema = &schemas[&name];
                let builds = constructed.contains(&name);
                if !builds {
                    item.attrs = item
                        .attrs
                        .iter()
                        .map(without_default)
                        .collect::<Result<_>>()?;
                }
                document(&mut item.attrs, &model_docs[&name], true);
                for (field, info) in item.fields.iter_mut().zip(&structs[&name]) {
                    document(&mut field.attrs, &field_doc(&name, info), false);
                }
                if builds {
                    constructors.push(constructor(analysis, schema, &item.ident, &structs[&name]));
                }
            }
            syn::Item::Enum(enum_item) => {
                let name = enum_item.ident.to_string();
                let schema = &schemas[&name];
                match &analysis.schemas[schema].schema_type {
                    SchemaType::StringEnum { values } => {
                        let docs = doc_comment(&[model_docs[&name].clone()]);
                        // A one-line placeholder, swapped for the laid-out invocation after printing.
                        rendered.insert(
                            format!("string_enum!({name});"),
                            string_enum(enum_item, &docs, values)?,
                        );
                        replacement = Some(syn::parse_str(&format!("string_enum!({name});"))?);
                        enums.push((name.clone(), values.clone()));
                        string_enums.insert(name);
                    }
                    SchemaType::ExtensibleEnum { .. } => {
                        return Err(format!("unexpected generator extensible enum {name}").into());
                    }
                    _ => {
                        document(&mut enum_item.attrs, &model_docs[&name], true);
                        for variant in &mut enum_item.variants {
                            let text = variant_doc(variant, &schemas);
                            document(&mut variant.attrs, &text, false);
                        }
                    }
                }
            }
            syn::Item::Type(item) => {
                let name = item.ident.to_string();
                let text = model_docs.get(&name).cloned().unwrap_or_default();
                document(&mut item.attrs, &text, false);
            }
            syn::Item::Impl(item) => {
                strip_docs(&mut item.attrs);
                for member in &mut item.items {
                    if let syn::ImplItem::Fn(function) = member {
                        strip_docs(&mut function.attrs);
                    }
                }
            }
            syn::Item::Mod(item) => strip_docs(&mut item.attrs),
            _ => {}
        }
        if let Some(replacement) = replacement {
            *item = replacement;
        }
    }
    // The macro defines each string enum with its methods; generator impls for them are dropped.
    syntax.items.retain(|item| match item {
        syn::Item::Impl(item) => match &*item.self_ty {
            syn::Type::Path(path) => !path
                .path
                .get_ident()
                .is_some_and(|ident| string_enums.contains(&ident.to_string())),
            _ => true,
        },
        _ => true,
    });
    syntax.items.extend(constructors);
    syntax
        .attrs
        .push(syn::parse_quote!(#![allow(clippy::too_many_arguments)]));
    let mut source = String::new();
    for line in prettyplease::unparse(&syntax).lines() {
        match rendered.remove(line) {
            Some(invocation) => source.push_str(&invocation),
            None => source.extend([line, "\n"]),
        }
    }
    if !rendered.is_empty() {
        return Err("a string enum placeholder was not printed".into());
    }
    Ok(Models {
        source,
        structs,
        enums,
    })
}

/// A standalone module: the string enum definition as text after the inner attributes, so it keeps its layout.
pub fn standalone(source: &str) -> String {
    let header = source
        .lines()
        .take_while(|line| line.starts_with("#!["))
        .map(|line| line.len() + 1)
        .sum::<usize>();
    format!("{}{STRING_ENUM}{}", &source[..header], &source[header..])
}

/// Test support that round-trips every known value of every string enum and one unknown value.
pub fn enum_fixture(enums: &[(String, Vec<String>)]) -> String {
    let mut source = String::from(
        "use roxyapi::types;\n\
         fn check<T>(values: &[&str], other: fn(String) -> T) -> usize where T: serde::de::DeserializeOwned + serde::Serialize + PartialEq + std::fmt::Debug + AsRef<str> {\n\
         let mut seen: Vec<T> = Vec::new();\n\
         for value in values {\n\
         let parsed: T = serde_json::from_value(serde_json::json!(value)).unwrap();\n\
         assert_ne!(parsed, other((*value).to_string()), \"{value} decoded as an unknown value\");\n\
         assert_eq!(parsed.as_ref(), *value);\n\
         assert_eq!(serde_json::to_value(&parsed).unwrap(), serde_json::json!(value));\n\
         assert!(!seen.contains(&parsed), \"{value} shares a variant\");\n\
         seen.push(parsed);\n\
         }\n\
         let unknown = \"value-outside-the-specification\";\n\
         let parsed: T = serde_json::from_value(serde_json::json!(unknown)).unwrap();\n\
         assert_eq!(parsed, other(unknown.to_string()));\n\
         assert_eq!(serde_json::to_value(&parsed).unwrap(), serde_json::json!(unknown));\n\
         values.len() + 1\n\
         }\n\
         pub fn every_enum_value() -> usize {\nlet mut count = 0;\n",
    );
    for (name, values) in enums {
        source.push_str(&format!(
            "count += check::<types::{name}>(&{values:?}, types::{name}::Other);\n"
        ));
    }
    source.push_str("count\n}\n");
    source
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn documentation_escapes_links_markup_urls_and_code() {
        let lines = doc_lines(
            "Use `cities[0].timezone` for [x] when a < b, see https://roxyapi.com/docs. # not a heading",
        );
        assert_eq!(
            lines,
            [
                "Use `cities[0].timezone` for \\[x\\] when a &lt; b, see <https://roxyapi.com/docs>. # not a heading"
            ]
        );
        assert_eq!(doc_lines("# Title"), ["\\# Title"]);
        assert_eq!(doc_lines("odd ` tick [x]"), ["odd \\` tick \\[x\\]"]);
        assert_eq!(
            doc_lines("Intro\n```\nlet x = [1];\n```\n    indented"),
            ["Intro", "```text", "let x = [1];", "```", "indented"]
        );
    }
}
