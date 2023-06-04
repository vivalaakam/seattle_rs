use std::env;

use chrono::Utc;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;

use collection::{Collection, CollectionField, Collections, FieldType, Storage};
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize)]
pub struct CollectionResponse {
    id: String,
    name: String,
}

#[tokio::test]
async fn collection_default() {
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

    let collections = Collections::new(instance).await;

    let collection = collections.storage.create_collection(
        table_name.to_string(),
        vec![
            CollectionField {
                name: "name".to_string(),
                field_type: FieldType::String,
                default: Some(Value::String("default_name".to_string())),
            }
        ]
    ).await.unwrap();

    collections.set_collection(&table_name, collection);

    let created_default = collections
        .insert(
            table_name.to_string(),
            json!({}),
        )
        .await;

    debug!("created_default {created_default:?}");

    assert_eq!(created_default.is_ok(), true);

    let created_default = created_default.unwrap();

    info!("created: {:?}", json!(created_default).to_string());

    let row_default = serde_json::from_value::<CollectionResponse>(created_default).unwrap();

    assert_eq!(row_default.name, "default_name");

    let created_exists = collections
        .insert(
            table_name.to_string(),
            json!({
                "name": "test"
            }),
        )
        .await;

    assert_eq!(created_exists.is_ok(), true);

    let created_exists = created_exists.unwrap();

    info!("created: {:?}", json!(created_exists).to_string());

    let row_exists = serde_json::from_value::<CollectionResponse>(created_exists).unwrap();

    assert_eq!(row_exists.name, "test");
}
