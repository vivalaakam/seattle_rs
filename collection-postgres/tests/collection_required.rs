use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};
use tracing_subscriber::filter::LevelFilter;

use collection::CollectionField;
use collection::FieldType;
use collection::{Collections, Storage};
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize)]
pub struct CollectionResponse {
    id: String,
    name: String,
}

#[tokio::test]
async fn collection_required() {
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

    let collection = collections
        .storage
        .create_collection(
            table_name.to_string(),
            vec![CollectionField {
                name: "name".to_string(),
                field_type: FieldType::String,
                default: None,
                required: Some(true),
            }],
        )
        .await
        .unwrap();

    collections.set_collection(&table_name, collection);

    let created_default = collections.insert(table_name.to_string(), json!({})).await;

    debug!("created_default {created_default:?}");

    assert_eq!(created_default.is_err(), true);

    assert_eq!(
        format!("{created_default:?}"),
        r#"Err(ValidateFields { collection: "Collection1", fields: ["name"] })"#
    );

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
