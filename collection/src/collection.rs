use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

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
