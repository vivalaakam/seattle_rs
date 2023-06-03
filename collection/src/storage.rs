use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::collection::{Collection, CollectionField};
use crate::storage_error::StorageError;
use crate::where_attr::Where;

#[async_trait]
pub trait Storage {
    async fn get_collections(&self) -> Result<Vec<Collection>, StorageError>;
    async fn get_collection(&self, collection_name: String) -> Result<Collection, StorageError>;
    async fn create_collection(
        &self,
        collection_name: String,
        fields: Vec<CollectionField>,
    ) -> Result<Collection, StorageError>;
    async fn remove_collection(&self, collection_name: String) -> Result<(), StorageError>;
    async fn insert_field_to_collection(
        &self,
        collection_name: String,
        field: CollectionField,
    ) -> Result<Collection, StorageError>;
    async fn remove_field_from_collection(
        &self,
        collection_name: String,
        field: CollectionField,
    ) -> Result<Collection, StorageError>;

    async fn insert_data_into_collection(
        &self,
        collection: String,
        data: Value,
    ) -> Result<Value, StorageError>;
    async fn update_data_into_collection(
        &self,
        collection: String,
        collection_id: String,
        data: Value,
    ) -> Result<Value, StorageError>;
    async fn delete_data_from_collection(
        &self,
        collection: String,
        collection_id: String,
    ) -> Result<(), StorageError>;
    async fn get_data_from_collection(
        &self,
        collection: String,
        collection_id: String,
    ) -> Result<Value, StorageError>;
    async fn list_data_from_collection(
        &self,
        collection: String,
        query: HashMap<String, Where>,
    ) -> Result<Vec<Value>, StorageError>;
}
