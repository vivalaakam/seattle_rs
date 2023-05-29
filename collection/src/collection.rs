use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::CollectionError;

const ID_FIELD: &str = "id";
const CREATED_AT_FIELD: &str = "created_at";
const UPDATED_AT_FIELD: &str = "updated_at";

const SKIP_FIELDS: [&'static str; 3] = [ID_FIELD, CREATED_AT_FIELD, UPDATED_AT_FIELD];

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
                !exists.contains(key) && !SKIP_FIELDS.contains(&key.as_str()) && !value.is_null()
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

    pub fn validate(&self, data: &Value) -> Result<(), CollectionError> {
        if !data.is_object() {
            return Err(CollectionError::CollectionInputData {
                collection: self.name.clone(),
            });
        }

        let exists: HashMap<String, FieldType> = HashMap::from_iter(
            self.fields
                .iter()
                .map(|field| (field.name.to_string(), field.field_type.clone())),
        );

        let fields = data
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| match exists.get(key) {
                None => None,
                Some(field) => match field {
                    FieldType::String | FieldType::TimeStamp => {
                        (!(value.is_string() || value.is_null())).then_some(key)
                    }
                    FieldType::Number => (!(value.is_number() || value.is_null())).then_some(key),
                    FieldType::Boolean => (!(value.is_boolean() || value.is_null())).then_some(key),
                    FieldType::Array => (!(value.is_array() || value.is_null())).then_some(key),
                    FieldType::Object => (!(value.is_object() || value.is_null())).then_some(key),
                },
            })
            .filter(|key| key.is_some())
            .map(|key| key.unwrap().to_string())
            .collect::<Vec<_>>();

        match fields.is_empty() {
            true => Ok(()),
            false => Err(CollectionError::ValidateFields {
                collection: self.name.clone(),
                fields,
            }),
        }
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
