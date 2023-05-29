use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use collection::{Collections, serialize_value};
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CollectionResponse {
    id: String,
    name: String,
    age: i32,
}

#[tokio::test]
async fn collection_get() {
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

    let mut collections = Collections::new(instance).await;

    let created = collections
        .insert(
            table_name.to_string(),
            json!({
                "name": "test",
                "age": 10,
            }),
        )
        .await;

    assert_eq!(created.is_ok(), true);

    let created = serialize_value(created.unwrap());

    info!("created: {created}");

    let row = serde_json::from_str::<CollectionResponse>(&created).unwrap();

    assert_eq!(row.name, "test");
    assert_eq!(row.age, 10);

    // check

    let check = collections
        .get(table_name.to_string(), row.id.to_string())
        .await;

    assert_eq!(check.is_ok(), true);

    let check = serialize_value(check.unwrap());

    let check_row = serde_json::from_str::<CollectionResponse>(&check).unwrap();

    assert_eq!(check_row, row);

    // check 2

    let check2 = collections
        .get(table_name.to_string(), "not-exists".to_string())
        .await;

    assert_eq!(check2.is_err(), true);

    assert_eq!(format!("{check2:?}"), r#"Err(StorageError { error: ValueNotFound { collection: "Collection1", id: "not-exists" } })"#);
}
