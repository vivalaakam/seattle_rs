use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::FieldType;

#[derive(Clone, Serialize, Deserialize)]
pub struct CollectionField {
    pub name: String,
    pub field_type: FieldType,
    pub default: Option<Value>,
    pub required: Option<bool>,
}
