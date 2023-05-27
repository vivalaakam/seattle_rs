use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing_subscriber::filter::LevelFilter;

use collection::{
    FieldType, StorageCollection, StorageCollectionField, StorageCollectionTrait, WhereAttr,
};
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize)]
struct CollectionResponse {
    id: String,
    name: String,
}

#[tokio::test]
async fn collection_string() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();

    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");

    let instance = StorePostgresql::new(database_url.as_str()).await;

    let table_name = "Collection1".to_string();

    cleanup_table(instance.get_pool(), &table_name).await;

    let schema = instance
        .create_collection(StorageCollection {
            name: table_name.to_string(),
            fields: vec![StorageCollectionField {
                name: "name".to_string(),
                field_type: FieldType::String,
            }],
            ..Default::default()
        })
        .await;

    assert_eq!(schema.is_ok(), true);

    let result = instance
        .insert_data_into_collection(table_name.to_string(), json!({ "name": "test"}))
        .await;

    assert_eq!(result.is_ok(), true);

    let row = serde_json::from_value::<CollectionResponse>(result.unwrap()).unwrap();
    assert_eq!(row.name, "test");

    let result = instance
        .update_data_into_collection(table_name.to_string(), row.id, json!({"name": "test2"}))
        .await;
    assert_eq!(result.is_ok(), true);

    let row = serde_json::from_value::<CollectionResponse>(result.unwrap()).unwrap();
    assert_eq!(row.name, "test2");

    let result = instance
        .insert_data_into_collection(table_name.to_string(), json!({ "name": "test3"}))
        .await;

    assert_eq!(result.is_ok(), true);

    let row = serde_json::from_value::<CollectionResponse>(result.unwrap()).unwrap();
    assert_eq!(row.name, "test3");

    let result = instance
        .insert_data_into_collection(table_name.to_string(), json!({ "name": "test4"}))
        .await;

    assert_eq!(result.is_ok(), true);

    let result4 = instance
        .insert_data_into_collection(table_name.to_string(), json!({ "name": "test5"}))
        .await;

    assert_eq!(result4.is_ok(), true);

    let row = serde_json::from_value::<CollectionResponse>(result4.unwrap()).unwrap();


    let mut where_clause = HashMap::new();

    where_clause.insert(
        "name".to_string(),
        WhereAttr::Eq(Value::String(row.name)),
    );

    let result_eq = instance
        .list_data_from_collection(table_name.to_string(), where_clause)
        .await;
    assert_eq!(result_eq.is_ok(), true);

    let result_eq = result_eq.unwrap();
    assert_eq!(result_eq.len(), 1);

    let row_eq_1 = serde_json::from_value::<CollectionResponse>(result_eq.first().unwrap().clone()).unwrap();

    assert_eq!(row.id, row_eq_1.id);
}
