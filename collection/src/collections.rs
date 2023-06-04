use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use anyhow::Result;
use serde_json::Value;

use crate::{Collection, Storage};
use crate::collection_error::CollectionError;
use crate::where_attr::Where;

#[derive(Clone)]
pub struct Collections<T> {
    pub collections: Arc<Mutex<HashMap<String, Collection>>>,
    pub storage: T,
}

impl<T> Collections<T> where T: Storage {
    pub fn get_collection(&self, key: &String) -> Option<Collection> {
        self.collections.lock().unwrap().get(key).cloned()
    }

    pub fn set_collection(&self, key: &String, value: Collection) {
        self.collections.lock().unwrap().insert(key.to_string(), value);
    }
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
            collections: Arc::new(Mutex::new(HashMap::from_iter(collections))),
            storage,
        }
    }

    pub fn get_storage(&self) -> &T {
        &self.storage
    }

    pub async fn insert(
        &self,
        collection_name: String,
        data: Value,
    ) -> Result<Value, CollectionError> {
        if !data.is_object() {
            return Err(CollectionError::CollectionInputData {
                collection: collection_name,
            });
        }

        let mut collection = self.get_collection(&collection_name);

        if collection.is_none() {
            let schema = self
                .storage
                .create_collection(collection_name.to_string(), vec![])
                .await;

            if schema.is_err() {
                return Err(CollectionError::StorageError {
                    error: schema.err().unwrap(),
                });
            }

            let mut coll = schema.unwrap();

            for field in coll.get_new_fields(&data) {
                coll = self
                    .storage
                    .insert_field_to_collection(collection_name.to_string(), field)
                    .await
                    .unwrap();
            }

            self.set_collection(&collection_name, coll);

            collection = self.get_collection(&collection_name);
        }

        let collection = collection.unwrap();

        collection.validate(&data)?;

        let data = collection.default_values(data);

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
        &self,
        collection_name: String,
        collection_id: String,
        data: Value,
    ) -> Result<Value, CollectionError> {
        if !data.is_object() {
            return Err(CollectionError::CollectionInputData {
                collection: collection_name,
            });
        }

        let collection = self.get_collection(&collection_name);

        if collection.is_none() {
            return Err(CollectionError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let mut collection = collection.unwrap();

        let fields = collection.get_new_fields(&data);

        if !fields.is_empty() {
            let mut coll = collection.clone();

            for field in fields {
                coll = self
                    .storage
                    .insert_field_to_collection(collection_name.to_string(), field)
                    .await
                    .unwrap();
            }

            self.set_collection(&collection_name, coll);

            collection = self.get_collection(&collection_name).unwrap()
        }

        collection.validate(&data)?;

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
    ) -> Result<Value, CollectionError> {
        let collection = self.get_collection(&collection_name);

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
            Ok(_) => Ok(Value::Null),
            Err(error) => Err(CollectionError::StorageError { error }),
        }
    }
    pub async fn get(
        &self,
        collection_name: String,
        collection_id: String,
    ) -> Result<Value, CollectionError> {
        let collection = self.get_collection(&collection_name);

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
        let collection = self.get_collection(&collection_name);

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
