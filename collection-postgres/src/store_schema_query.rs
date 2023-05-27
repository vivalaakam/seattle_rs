use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, types::Json};

use collection::StorageCollection;

#[derive(FromRow)]
pub struct StoreCollectionQuery {
    name: String,
    fields: Json<Value>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Into<StorageCollection> for StoreCollectionQuery {
    fn into(self) -> StorageCollection {
        StorageCollection {
            name: self.name,
            fields: serde_json::from_value(self.fields.0).unwrap(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
