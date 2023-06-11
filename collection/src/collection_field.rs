use crate::FieldType;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct CollectionField {
    pub name: String,
    pub field_type: FieldType,
    pub default: Option<Value>,
    pub required: Option<bool>,
}
