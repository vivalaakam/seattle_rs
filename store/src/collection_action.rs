use serde::{Deserialize, Serialize};
use serde_json::Value;

use collection::{CollectionError, Storage};

use crate::App;

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum CollectionAction {
    Create {
        collection: String,
        data: Value,
    },
    Update {
        collection: String,
        identifier: String,
        data: Value,
    },
    Delete {
        collection: String,
        identifier: String,
    },
    Get {
        collection: String,
        identifier: String,
    },
}

impl CollectionAction {
    pub async fn perform<T>(&self, app: &App<T>) -> Result<Value, CollectionError> where T: Storage {
        match self {
            CollectionAction::Create { collection, data } => {
                app
                    .get_collections()
                    .insert(collection.to_string(), data.clone())
                    .await
            }
            CollectionAction::Update { collection, identifier, data } => {
                app
                    .get_collections()
                    .update(collection.to_string(), identifier.to_string(), data.clone())
                    .await
            }
            CollectionAction::Delete { collection, identifier } => {
                app
                    .get_collections()
                    .delete(collection.to_string(), identifier.to_string())
                    .await
            }
            CollectionAction::Get { collection, identifier } => {
                app
                    .get_collections()
                    .get(collection.to_string(), identifier.to_string())
                    .await
            }
        }
    }
}
