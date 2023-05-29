use std::collections::HashMap;

use anyhow::Result;
use serde_json::Value;

use crate::{Collection, Storage};
use crate::collection_error::CollectionError;
use crate::where_attr::Where;

pub struct Collections<T> {
    pub collections: HashMap<String, Collection>,
    pub storage: T,
}

impl<T> Collections<T>
    where
        T: Storage,
{
    pub async fn new(storage: T) -> Self {
        let collections = storage
            .get_collections()
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|collection| (collection.name.clone(), collection));

        Self {
            collections: HashMap::from_iter(collections),
            storage,
        }
    }

    pub async fn insert(
        &mut self,
        collection_name: String,
        data: Value,
    ) -> Result<Value, CollectionError> {
        if !data.is_object() {
            return Err(CollectionError::CollectionInputData {
                collection: collection_name,
            });
        }

        if !self.collections.contains_key(&collection_name) {
            let schema = self
                .storage
                .create_collection(Collection {
                    name: collection_name.to_string(),
                    fields: vec![],
                    ..Default::default()
                })
                .await;

            if schema.is_err() {
                return Err(CollectionError::StorageError {
                    error: schema.err().unwrap(),
                });
            }

            let mut collection = schema.unwrap();

            for field in collection.get_new_fields(&data) {
                collection = self
                    .storage
                    .insert_field_to_collection(collection_name.to_string(), field)
                    .await
                    .unwrap();
            }

            self.collections
                .insert(collection_name.to_string(), collection);
        }

        match self
            .storage
            .insert_data_into_collection(collection_name, data)
            .await
        {
            Ok(data) => Ok(data),
            Err(error) => Err(CollectionError::StorageError { error }),
        }
    }

    pub async fn update(
        &mut self,
        collection_name: String,
        collection_id: String,
        data: Value,
    ) -> Result<Value, CollectionError> {
        if !data.is_object() {
            return Err(CollectionError::CollectionInputData {
                collection: collection_name,
            });
        }

        let collection = self.collections.get(&collection_name);

        if collection.is_none() {
            return Err(CollectionError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let collection = collection.unwrap();

        let fields = collection.get_new_fields(&data);

        if !fields.is_empty() {
            let mut collection = collection.clone();

            for field in fields {
                collection = self
                    .storage
                    .insert_field_to_collection(collection_name.to_string(), field)
                    .await
                    .unwrap();
            }

            self.collections
                .insert(collection_name.to_string(), collection);
        }

        match self
            .storage
            .update_data_into_collection(collection_name, collection_id, data)
            .await
        {
            Ok(data) => Ok(data),
            Err(error) => Err(CollectionError::StorageError { error }),
        }
    }
    pub async fn delete(
        &self,
        collection_name: String,
        collection_id: String,
    ) -> Result<(), CollectionError> {
        let collection = self.collections.get(&collection_name);

        if collection.is_none() {
            return Err(CollectionError::CollectionNotFound {
                collection: collection_name,
            });
        }

        match self
            .storage
            .delete_data_from_collection(collection_name, collection_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(error) => Err(CollectionError::StorageError { error }),
        }
    }
    pub async fn get(
        &self,
        collection_name: String,
        collection_id: String,
    ) -> Result<Value, CollectionError> {
        let collection = self.collections.get(&collection_name);

        if collection.is_none() {
            return Err(CollectionError::CollectionNotFound {
                collection: collection_name,
            });
        }

        match self
            .storage
            .get_data_from_collection(collection_name, collection_id)
            .await
        {
            Ok(data) => Ok(data),
            Err(error) => Err(CollectionError::StorageError { error }),
        }
    }
    pub async fn list(
        &self,
        collection_name: String,
        query: Value,
    ) -> Result<Vec<Value>, CollectionError> {
        let collection = self.collections.get(&collection_name);

        if collection.is_none() {
            return Err(CollectionError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let fields = query
            .as_object()
            .unwrap()
            .iter()
            .map(|(key, value)| {
                if !value.is_object() {
                    (
                        key.to_string(),
                        Where {
                            eq: Some(value.clone()),
                            ..Default::default()
                        },
                    )
                } else {
                    (
                        key.to_string(),
                        serde_json::from_value(value.clone()).unwrap(),
                    )
                }
            })
            .collect::<Vec<_>>();

        let collection_query = HashMap::from_iter(fields);

        match self
            .storage
            .list_data_from_collection(collection_name, collection_query)
            .await
        {
            Ok(data) => Ok(data),
            Err(error) => Err(CollectionError::StorageError { error }),
        }
    }
}
