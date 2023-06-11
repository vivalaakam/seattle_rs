use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde_json::{Map, Value};

use crate::collection_field::CollectionField;
use crate::CollectionError;
use crate::field_type::FieldType;

const ID_FIELD: &str = "id";
const CREATED_AT_FIELD: &str = "created_at";
const UPDATED_AT_FIELD: &str = "updated_at";

const SKIP_FIELDS: [&str; 3] = [ID_FIELD, CREATED_AT_FIELD, UPDATED_AT_FIELD];

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
                default: None,
                required: None,
                field_type: value.clone().into(),
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
            .map(|(key, value)| {
                exists
                    .get(key)
                    .map(|field| (!field.is_maybe_exists(value)).then_some(key))
                    .unwrap()
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

    pub fn required_values(&self, data: &Value, only_exists: bool) -> Result<(), CollectionError> {
        let fields = self
            .fields
            .iter()
            .filter(|f| f.required.unwrap_or_default())
            .filter(|f| {
                if only_exists {
                    data.get(f.name.to_string()).is_some()
                } else {
                    true
                }
            })
            .filter(|f| {
                data.get(f.name.to_string())
                    .unwrap_or(&Value::Null)
                    .is_null()
            })
            .map(|field| field.name.to_string())
            .collect::<Vec<_>>();

        match fields.is_empty() {
            true => Ok(()),
            false => Err(CollectionError::ValidateFields {
                collection: self.name.clone(),
                fields,
            }),
        }
    }

    pub fn default_values(&self, data: Value) -> Value {
        let fields = self.fields.clone().into_iter().map(|f| {
            let val = data.get(f.name.to_string()).unwrap_or(&Value::Null).clone();

            if val.is_null() {
                (f.name, f.default.unwrap_or(Value::Null))
            } else {
                (f.name, val)
            }
        });

        Value::Object(Map::from_iter(fields))
    }
}
