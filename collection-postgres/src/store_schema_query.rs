use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{types::Json, FromRow};

use vivalaakam_seattle_collection::Collection;

#[derive(FromRow)]
pub struct StoreCollectionQuery {
    name: String,
    fields: Json<Value>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<StoreCollectionQuery> for Collection {
    fn from(val: StoreCollectionQuery) -> Self {
        Collection {
            name: val.name,
            fields: serde_json::from_value(val.fields.0).unwrap(),
            created_at: val.created_at,
            updated_at: val.updated_at,
        }
    }
}
