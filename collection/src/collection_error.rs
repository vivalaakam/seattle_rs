use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StorageError;

#[derive(Error, Debug, PartialEq, Serialize, Deserialize)]
pub enum CollectionError {
    #[error("Storage error: {error}")]
    StorageError { error: StorageError },
    #[error("Data must be an object {collection}")]
    CollectionInputData { collection: String },
    #[error("Collection not found: {collection}")]
    CollectionNotFound { collection: String },
    #[error("Invalid field data: {collection} - {fields:?}")]
    ValidateFields { collection: String, fields: Vec<String> },
}
