use std::collections::HashMap;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use crate::storage_error::StorageError;
use crate::storage_collection::{StorageCollection, StorageCollectionField};
use crate::WhereAttr;

#[async_trait]
pub trait StorageCollectionTrait {
    async fn get_collections(&self) -> Result<Vec<StorageCollection>, StorageError>;
    async fn get_collection(&self, collection_name: String) -> Result<StorageCollection, StorageError>;
    async fn create_collection(&self, collection: StorageCollection) -> Result<StorageCollection, StorageError>;
    async fn remove_collection(&self, collection: StorageCollection) -> Result<(), StorageError>;
    async fn insert_field_to_collection(
        &self,
        collection: StorageCollection,
        field: StorageCollectionField,
    ) -> Result<StorageCollection, StorageError>;
    async fn remove_field_from_collection(
        &self,
        collection: StorageCollection,
        field: StorageCollectionField,
    ) -> Result<StorageCollection, StorageError>;

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
        query: HashMap<String, WhereAttr>,
    ) -> Result<Vec<Value>, StorageError>;
}
