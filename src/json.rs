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
            $( json!($value) ),*
        ])
    }};
    ({ $($key:tt : $value:tt),* } $(,)?) => {{
        crate::json::Json::Object(std::collections::HashMap::from_iter([
            $( ($key.into(), json!($value)) ),*
        ]))
    }};
    (null) => {
        crate::json::Json::Null
    };
    ($value:expr) => {
        crate::json::Json::from($value)
    };
}

#[test]
fn should_work_with_primitives() {
    assert_eq!(json!(null), Json::Null);
    assert_eq!(json!(true), Json::Boolean(true));
    assert_eq!(json!(123.0), Json::Number(123.0));
    assert_eq!(json!(123), Json::Number(123.0));
    assert_eq!(json!("Hello"), Json::String("Hello".into()));
}

#[test]
fn should_work_with_arrays_of_primitives() {
    assert_eq!(
        json!([1, 2, "Hello"]),
        Json::Array(vec![
            Json::Number(1.0),
            Json::Number(2.0),
            Json::String("Hello".to_string()),
        ])
    );
}

#[test]
fn should_work_with_objects() {
    assert_eq!(
        json!({ "a": "Hello", "b": 123 }),
        Json::Object(HashMap::from_iter([
            ("a".into(), Json::String("Hello".into())),
            ("b".into(), Json::Number(123.0))
        ]))
    );
}

#[test]
fn should_work_with_arrays_of_objects() {
    assert_eq!(
        json!([{"a": 1}, {"b": 2}]),
        Json::Array(vec![
            Json::Object(HashMap::from_iter([("a".into(), Json::Number(1.0))])),
            Json::Object(HashMap::from_iter([("b".into(), Json::Number(2.0))])),
        ])
    );
}
