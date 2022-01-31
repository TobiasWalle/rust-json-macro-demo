#![allow(dead_code, unused_macros)]
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

macro_rules! impl_from {
    ($from_type:ty, $enum_variant:expr) => {
        impl From<$from_type> for Json {
            fn from(value: $from_type) -> Self {
                $enum_variant(value.into())
            }
        }
    };
}

impl_from!(String, Json::String);
impl_from!(&str, Json::String);
impl_from!(bool, Json::Boolean);
impl_from!(f64, Json::Number);
impl_from!(i32, Json::Number);

#[macro_export]
macro_rules! json {
    ([ $($value:tt),* $(,)? ]) => {{
        crate::json::Json::Array(vec![
            $(json!($value)),*
        ])
    }};
    ({ $($key:tt : $value:tt),* } $(,)?) => {{
        let mut values = std::collections::HashMap::new();
        $(
        values.insert($key.into(), json!($value));
        )*
        crate::json::Json::Object(values)
    }};
    (null) => {
        crate::json::Json::Null
    };
    ($value:expr) => {
        crate::json::Json::from($value)
    };
}

#[test]
fn test_values() {
    assert_eq!(json!(null), Json::Null);
    assert_eq!(json!(true), Json::Boolean(true));
    assert_eq!(json!(123.0), Json::Number(123.0));
    assert_eq!(json!(123), Json::Number(123.0));
    assert_eq!(json!("Hello"), Json::String("Hello".to_string()));
}

#[test]
fn test_object() {
    let mut values = HashMap::new();
    values.insert("a".into(), Json::String("Hello".into()));
    values.insert("b".into(), Json::Number(123.0));
    let my_object = Json::Object(values);
    assert_eq!(json!({ "a": "Hello", "b": 123 }), my_object);
}

#[test]
fn test_array() {
    assert_eq!(
        json!([1, 2, "3"]),
        Json::Array(vec![
            Json::Number(1.0),
            Json::Number(2.0),
            Json::String("3".into())
        ])
    );
}

#[test]
fn test_object_array() {
    assert_eq!(
        json!([{ "name": "Susan" }, { "name": "Karl" }]),
        Json::Array(vec![
            Json::Object(HashMap::from_iter([(
                "name".into(),
                Json::String("Susan".into())
            ),])),
            Json::Object(HashMap::from_iter([(
                "name".into(),
                Json::String("Karl".into())
            ),])),
        ])
    );
}
