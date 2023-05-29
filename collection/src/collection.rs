use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const ID_FIELD: &str = "id";
const CREATED_AT_FIELD: &str = "created_at";
const UPDATED_AT_FIELD: &str = "updated_at";

#[derive(Clone, Default)]
pub struct Collection {
    pub name: String,
    pub fields: Vec<CollectionField>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Collection {
    pub fn get_field(&self, key: &String) -> Option<&CollectionField> {
        self.fields.iter().find(|f| f.name == *key)
    }

    pub fn get_new_fields(&self, data: &Value) -> Vec<CollectionField> {
        let exists = self
            .fields
            .iter()
            .map(|field| field.name.to_string())
            .collect::<Vec<_>>();

        data.as_object()
            .unwrap()
            .iter()
            .filter(|(key, value)| {
                !exists.contains(key)
                    && key.as_str() != ID_FIELD
                    && key.as_str() != CREATED_AT_FIELD
                    && key.as_str() != UPDATED_AT_FIELD
                    && !value.is_null()
            })
            .map(|(key, value)| CollectionField {
                name: key.to_string(),
                field_type: match value {
                    Value::String(_) => FieldType::String,
                    Value::Number(_) => FieldType::Number,
                    Value::Bool(_) => FieldType::Boolean,
                    Value::Array(_) => FieldType::Array,
                    Value::Object(_) => FieldType::Object,
                    Value::Null => FieldType::Object,
                },
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CollectionField {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    TimeStamp,
}
