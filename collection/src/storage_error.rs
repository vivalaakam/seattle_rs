use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Serialize, Deserialize)]
pub enum StorageError {
    #[error("Create table: {collection}")]
    CollectionCreateTable { collection: String },
    #[error("Insert collection {collection} failed")]
    CollectionCreate { collection: String },
    #[error("Remove collection {collection} failed")]
    CollectionRemove { collection: String },
    #[error("Schema not found: {collection}")]
    CollectionNotFound { collection: String },
    #[error("Field {field} exists in collection {collection}")]
    CollectionFieldExists { collection: String, field: String },
    #[error("Alter table {collection} failed {field}")]
    CollectionAlterTable { collection: String, field: String },
    #[error("Alter table {collection} drop {field} failed")]
    CollectionFieldRemove { collection: String, field: String },
    #[error("Value not found {collection} : {id}")]
    ValueNotFound { collection: String, id: String },
}
