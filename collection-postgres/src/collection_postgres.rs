use std::collections::HashMap;
use std::iter;

use async_trait::async_trait;
use serde_json::{json, Value};
use sqlx::postgres::PgArguments;
use sqlx::Arguments;
use sqlx::{PgPool, Pool, Postgres, Row};
use tracing::{debug, error, info};

use collection::{make_id, Collection, CollectionField, FieldType, Storage, StorageError, Where};

use crate::add_value_into_args::add_value_into_args;
use crate::serialize_pg_row::serialize_pg_row;
use crate::store_schema_query::StoreCollectionQuery;

pub struct StorePostgresql {
    pub pool: Pool<Postgres>,
}

const ID_FIELD: &str = "id";
const CREATED_AT_FIELD: &str = "created_at";
const UPDATED_AT_FIELD: &str = "updated_at";

impl StorePostgresql {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPool::connect(database_url)
            .await
            .expect("postgresql fails");

        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("postgres migration failed");

        Self { pool }
    }

    fn query_field_to_collection(schema: String, field: &CollectionField) -> String {
        match field.field_type {
            FieldType::String => format!(r#"ALTER TABLE "{schema}" ADD "{}" text;"#, field.name),
            FieldType::Number => format!(
                r#"ALTER TABLE "{schema}" ADD "{}" double precision;"#,
                field.name
            ),
            FieldType::Boolean => {
                format!(r#"ALTER TABLE "{schema}" ADD "{}" boolean;"#, field.name)
            }
            FieldType::Array => format!(r#"ALTER TABLE "{schema}" ADD "{}" jsonb;"#, field.name),
            FieldType::Object => format!(r#"ALTER TABLE "{schema}" ADD "{}" jsonb;"#, field.name),
            FieldType::TimeStamp => {
                format!(
                    r#"ALTER TABLE "{schema}" ADD "{}" timestamptz;"#,
                    field.name
                )
            }
        }
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

#[async_trait]
impl Storage for StorePostgresql {
    async fn get_collections(&self) -> anyhow::Result<Vec<Collection>, StorageError> {
        let schemas: Vec<StoreCollectionQuery> =
            sqlx::query_as(r#"SELECT * FROM storage_collection_schema"#)
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default();

        Ok(schemas.into_iter().map(|schema| schema.into()).collect())
    }

    async fn get_collection(
        &self,
        collection_name: String,
    ) -> anyhow::Result<Collection, StorageError> {
        let schema: Option<StoreCollectionQuery> =
            sqlx::query_as(r#"SELECT * FROM storage_collection_schema WHERE name = $1"#)
                .bind(collection_name.to_string())
                .fetch_optional(&self.pool)
                .await
                .unwrap_or_default();

        if schema.is_none() {
            return Err(StorageError::CollectionNotFound {
                collection: collection_name,
            });
        }

        Ok(schema.unwrap().into())
    }

    async fn create_collection(
        &self,
        collection: Collection,
    ) -> anyhow::Result<Collection, StorageError> {
        let collection_name = collection.name.to_string();
        let mut transaction = self.pool.begin().await.expect("transaction failed");

        let query = format!(r#"
        CREATE TABLE IF NOT EXISTS "{collection_name}"
            (
                id          VARCHAR                  NOT NULL PRIMARY KEY,
                created_at  TIMESTAMP with time zone NOT NULL,
                updated_at  TIMESTAMP with time zone NOT NULL
            );
        "#);

        let create_table = sqlx::query(query.as_str()).execute(&mut transaction).await;

        if create_table.is_err() {
            error!("create_table: {:?}", create_table.unwrap_err());
            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionCreateTable {
                collection: collection_name.to_string(),
            });
        }

        let mut id_exists = false;
        let mut created_at_exists = false;
        let mut updated_at_exists = false;

        let mut fields = collection.fields.clone();

        for field in &fields {
            match field.name.as_str() {
                ID_FIELD => {
                    id_exists = true;
                    break;
                }
                CREATED_AT_FIELD => {
                    created_at_exists = true;
                    break;
                }
                UPDATED_AT_FIELD => {
                    updated_at_exists = true;
                    break;
                }
                _ => {
                    let query = Self::query_field_to_collection(collection.name.to_string(), field);
                    let create_field = sqlx::query(query.as_str()).execute(&mut transaction).await;

                    if create_field.is_err() {
                        transaction.rollback().await.unwrap();
                        return Err(StorageError::CollectionAlterTable {
                            collection: collection.name,
                            field: field.name.to_string(),
                        });
                    }
                }
            }
        }

        if !id_exists {
            fields.push(CollectionField {
                name: ID_FIELD.to_string(),
                field_type: FieldType::String,
            });
        }

        if !created_at_exists {
            fields.push(CollectionField {
                name: CREATED_AT_FIELD.to_string(),
                field_type: FieldType::TimeStamp,
            });
        }

        if !updated_at_exists {
            fields.push(CollectionField {
                name: UPDATED_AT_FIELD.to_string(),
                field_type: FieldType::TimeStamp,
            });
        }

        let fields = json!(fields).to_string();

        debug!(
            "create collection: {} with fields: {fields}",
            collection.name
        );

        let insert_collection = sqlx::query(r#"INSERT INTO storage_collection_schema ( name, fields, created_at, updated_at ) VALUES ( $1 , $2::jsonb, NOW(), NOW() )"#)
            .bind(collection.name.to_string())
            .bind(fields)
            .execute(&mut transaction)
            .await;

        if insert_collection.is_err() {
            error!("insert_collection: {:?}", insert_collection.unwrap_err());

            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionCreate {
                collection: collection.name,
            });
        }

        transaction.commit().await.unwrap();

        self.get_collection(collection.name.to_string()).await
    }

    async fn remove_collection(&self, collection: Collection) -> Result<(), StorageError> {
        let collection_name = collection.name.to_string();
        let collection = self.get_collection(collection_name.to_string()).await;

        if collection.is_err() {
            return Err(StorageError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let mut transaction = self.pool.begin().await.expect("transaction failed");

        let drop_table = sqlx::query(r#"DROP TABLE storage_collection_schema"#)
            .execute(&mut transaction)
            .await;

        if drop_table.is_err() {
            error!("drop_table: {:?}", drop_table.unwrap_err());
            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionCreateTable {
                collection: collection_name.to_string(),
            });
        }

        let remove_collection =
            sqlx::query(r#"DELETE FROM storage_collection_schema WHERE name LIKE $1"#)
                .bind(collection_name.to_string())
                .execute(&mut transaction)
                .await;

        if remove_collection.is_err() {
            error!("remove_collection: {:?}", remove_collection.unwrap_err());

            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionRemove {
                collection: collection_name.to_string(),
            });
        }

        transaction.commit().await.unwrap();

        Ok(())
    }

    async fn insert_field_to_collection(
        &self,
        collection_name: String,
        field: CollectionField,
    ) -> anyhow::Result<Collection, StorageError> {
        let collection = self.get_collection(collection_name.to_string()).await;

        if collection.is_err() {
            return Err(StorageError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let collection = collection.unwrap();

        if collection.fields.iter().any(|f| f.name == field.name) {
            return Err(StorageError::CollectionFieldExists {
                collection: collection.name,
                field: field.name,
            });
        }

        let mut transaction = self.pool.begin().await.expect("transaction failed");

        let query = Self::query_field_to_collection(collection.name.to_string(), &field);
        let create_field = sqlx::query(query.as_str()).execute(&mut transaction).await;

        if create_field.is_err() {
            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionAlterTable {
                collection: collection.name,
                field: field.name.to_string(),
            });
        }

        let mut fields = collection.fields.clone();

        fields.push(field);

        let update_collection = sqlx::query(
            r#"UPDATE storage_collection_schema SET fields = $1::jsonb, updated_at = NOW() WHERE name = $2;"#,
        )
            .bind(json!(fields).to_string())
            .bind(collection_name.to_string())
            .execute(&mut transaction)
            .await;

        if update_collection.is_err() {
            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionCreate {
                collection: collection.name,
            });
        }

        transaction.commit().await.unwrap();

        self.get_collection(collection.name.to_string()).await
    }

    async fn remove_field_from_collection(
        &self,
        collection_name: String,
        field: CollectionField,
    ) -> anyhow::Result<Collection, StorageError> {
        let collection = self.get_collection(collection_name.to_string()).await;

        if collection.is_err() {
            return Err(StorageError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let collection = collection.unwrap();

        let position = collection.fields.iter().position(|f| f.name == field.name);

        if position.is_none() {
            return Err(StorageError::CollectionFieldExists {
                collection: collection.name,
                field: field.name,
            });
        }

        let mut transaction = self.pool.begin().await.expect("transaction failed");

        let query = format!(
            r#"ALTER TABLE "{}" DROP COLUMN "{}";"#,
            collection.name, field.name
        );
        let remove_field = sqlx::query(query.as_str()).execute(&mut transaction).await;

        if remove_field.is_err() {
            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionFieldRemove {
                collection: collection_name,
                field: field.name.to_string(),
            });
        }

        let mut fields = collection.fields.clone();
        fields.remove(position.unwrap());

        let fields = json!(fields).to_string();

        debug!(
            "remove_field_from_create_collection: {} with fields: {fields}",
            collection_name
        );

        let update_collection = sqlx::query(
            r#"UPDATE storage_collection_schema  SET fields = $1::jsonb, updated_at = NOW() WHERE name = $2;"#,
        )
            .bind(fields)
            .bind(collection_name.to_string())
            .execute(&mut transaction)
            .await;

        if update_collection.is_err() {
            transaction.rollback().await.unwrap();
            return Err(StorageError::CollectionCreate {
                collection: collection_name,
            });
        }

        transaction.commit().await.unwrap();

        self.get_collection(collection_name.to_string()).await
    }

    async fn insert_data_into_collection(
        &self,
        collection_name: String,
        data: Value,
    ) -> anyhow::Result<Value, StorageError> {
        let collection = self.get_collection(collection_name.to_string()).await;

        if collection.is_err() {
            return Err(StorageError::CollectionNotFound {
                collection: collection_name,
            });
        }

        let collection = collection.unwrap();

        let mut arguments = PgArguments::default();

        let mut insert_fields = vec![];
        let mut insert_indexes = vec![];
        let mut counter = 1;
        for field in &collection.fields {
            match field.name.as_str() {
                CREATED_AT_FIELD | UPDATED_AT_FIELD => {
                    continue;
                }
                ID_FIELD => {
                    let id = match data.get(field.name.as_str()) {
                        Some(v) => v.as_str().unwrap_or(&make_id(10)).to_string(),
                        None => make_id(10),
                    };

                    insert_fields.push(field.name.to_string());
                    insert_indexes.push(format!("${counter}"));
                    arguments.add(id);
                    counter += 1;
                }
                _ => {
                    let v = data.get(field.name.as_str()).cloned();

                    if let Some(v) = v {
                        insert_fields.push(field.name.to_string());
                        insert_indexes.push(format!("${counter}"));
                        add_value_into_args(field, &v, &mut arguments);
                        counter += 1;
                    }
                }
            }
        }

        insert_fields.push(CREATED_AT_FIELD.to_string());
        insert_fields.push(UPDATED_AT_FIELD.to_string());
        insert_indexes.push("NOW()".to_string());
        insert_indexes.push("NOW()".to_string());

        let insert_fields = insert_fields.join(", ");
        let insert_indexes = insert_indexes.join(", ");

        let rec = sqlx::query_with(format!(r#"INSERT INTO "{collection_name}" ({insert_fields}) VALUES ({insert_indexes}) RETURNING id"#).as_str(), arguments)
            .fetch_one(&self.pool)
            .await
            .expect("create error");

        let collection_id = rec.get::<String, _>("id");

        info!("insert_data_into_collection: {collection_name} with id: {collection_id}");

        self.get_data_from_collection(collection_name, collection_id)
            .await
    }

    async fn update_data_into_collection(
        &self,
        collection_name: String,
        collection_id: String,
        data: Value,
    ) -> anyhow::Result<Value, StorageError> {
        let collection = self
            .get_collection(collection_name.to_string())
            .await
            .unwrap();
        let mut arguments = PgArguments::default();

        let mut update_fields = vec![];
        let mut counter = 1;
        for field in &collection.fields {
            match field.name.as_str() {
                CREATED_AT_FIELD | UPDATED_AT_FIELD => {
                    continue;
                }
                ID_FIELD => {
                    if let Some(id) = data.get(field.name.as_str()) {
                        if !id.is_null() {
                            update_fields.push(format!("{} = ${counter}", field.name));
                            arguments.add(id);
                            counter += 1;
                        }
                    }
                }
                _ => {
                    if let Some(v) = data.get(field.name.as_str()).cloned() {
                        update_fields.push(format!("{} = ${counter}", field.name));
                        add_value_into_args(field, &v, &mut arguments);
                        counter += 1;
                    }
                }
            }
        }
        if !update_fields.is_empty() {
            update_fields.push(format!("{UPDATED_AT_FIELD} = NOW()"));
            arguments.add(collection_id.to_string());
            let update_fields = update_fields.join(", ");

            let _rec = sqlx::query_with(
                format!(r#"UPDATE "{collection_name}" SET {update_fields} WHERE id = ${counter}"#,)
                    .as_str(),
                arguments,
            )
            .execute(&self.pool)
            .await
            .expect("update error");

            info!("update_into_collection: {collection_name} with id: {collection_id}");
        }

        self.get_data_from_collection(collection_name, collection_id)
            .await
    }

    async fn delete_data_from_collection(
        &self,
        collection: String,
        collection_id: String,
    ) -> anyhow::Result<(), StorageError> {
        let query = format!(r#"DELETE FROM "{collection}" WHERE id = $1"#);

        let _rec = sqlx::query(query.as_str())
            .bind(collection_id)
            .execute(&self.pool)
            .await
            .expect("delete error");

        Ok(())
    }

    async fn get_data_from_collection(
        &self,
        collection_name: String,
        collection_id: String,
    ) -> anyhow::Result<Value, StorageError> {
        let query = format!(r#"SELECT * FROM "{collection_name}" WHERE id = $1"#);

        let value = sqlx::query(query.as_str())
            .bind(collection_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .unwrap_or_default();

        if value.is_none() {
            return Err(StorageError::ValueNotFound {
                collection: collection_name,
                id: collection_id,
            });
        }

        let collection = self
            .get_collection(collection_name.to_string())
            .await
            .unwrap();

        Ok(serialize_pg_row(&collection, value.unwrap()))
    }

    async fn list_data_from_collection(
        &self,
        collection: String,
        query: HashMap<String, Where>,
    ) -> anyhow::Result<Vec<Value>, StorageError> {
        let collection = self.get_collection(collection.to_string()).await.unwrap();

        let mut arguments = PgArguments::default();
        let mut where_query = vec![];
        let mut counter = 1;
        for (key, value) in query {
            if let Some(field) = collection.get_field(&key) {
                if let Some(eq) = value.eq {
                    where_query.push(format!(r#""{key}" = ${counter}"#));
                    add_value_into_args(field, &eq, &mut arguments);
                    counter += 1;
                }

                if let Some(ne) = value.ne {
                    where_query.push(format!(r#""{key}" != ${counter}"#));
                    add_value_into_args(field, &ne, &mut arguments);
                    counter += 1;
                }

                if let Some(gt) = value.gt {
                    where_query.push(format!(r#""{key}" > ${counter}"#));
                    add_value_into_args(field, &gt, &mut arguments);
                    counter += 1;
                }

                if let Some(gte) = value.gte {
                    where_query.push(format!(r#""{key}" >= ${counter}"#));
                    add_value_into_args(field, &gte, &mut arguments);
                    counter += 1;
                }

                if let Some(lt) = value.lt {
                    where_query.push(format!(r#""{key}" < ${counter}"#));
                    add_value_into_args(field, &lt, &mut arguments);
                    counter += 1;
                }

                if let Some(lte) = value.lte {
                    where_query.push(format!(r#""{key}" <= ${counter}"#));
                    add_value_into_args(field, &lte, &mut arguments);
                    counter += 1;
                }

                if let Some(in_) = value.in_ {
                    let in_query = iter::repeat(in_.len())
                        .take(in_.len())
                        .enumerate()
                        .map(|a| format!("${}", counter + a.0))
                        .collect::<Vec<_>>()
                        .join(", ");

                    where_query.push(format!(r#""{key}" = ANY(ARRAY[{in_query}])"#));
                    counter += in_.len();

                    for v in in_ {
                        add_value_into_args(field, &v, &mut arguments);
                    }
                }

                if let Some(nin) = value.nin {
                    let in_query = iter::repeat(nin.len())
                        .take(nin.len())
                        .enumerate()
                        .map(|a| format!("${}", counter + a.0))
                        .collect::<Vec<_>>()
                        .join(", ");

                    where_query.push(format!(r#"NOT("{key}" = ANY(ARRAY[{in_query}]))"#));
                    counter += nin.len();
                    for v in nin {
                        add_value_into_args(field, &v, &mut arguments);
                    }
                }
            }
        }

        let where_query = where_query.join(" AND ");

        let query = format!(
            r#"SELECT * FROM "{collection}" WHERE {where_query}"#,
            collection = collection.name,
            where_query = where_query
        );

        let values = sqlx::query_with(query.as_str(), arguments)
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default();

        let mut result = vec![];
        for value in values {
            result.push(serialize_pg_row(&collection, value));
        }

        Ok(result)
    }
}
