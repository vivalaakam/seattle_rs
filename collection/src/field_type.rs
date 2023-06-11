use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    TimeStamp,
}

impl FieldType {
    pub fn is_maybe_exists(&self, value: &Value) -> bool {
        match self {
            FieldType::String | FieldType::TimeStamp => value.is_string() || value.is_null(),
            FieldType::Number => value.is_number() || value.is_null(),
            FieldType::Boolean => value.is_boolean() || value.is_null(),
            FieldType::Array => value.is_array() || value.is_null(),
            FieldType::Object => value.is_object() || value.is_null(),
        }
    }
}

impl From<Value> for FieldType {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => FieldType::String,
            Value::Number(_) => FieldType::Number,
            Value::Bool(_) => FieldType::Boolean,
            Value::Array(_) => FieldType::Array,
            Value::Object(_) => FieldType::Object,
            Value::Null => FieldType::Object,
        }
    }
}
