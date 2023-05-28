use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use collection::Collections;
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize)]
pub struct CollectionResponse {
    id: String,
    name: String,
}

#[tokio::test]
async fn collections_delete() {
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
            }),
        )
        .await;

    assert_eq!(created.is_ok(), true);

    let created = created.unwrap();

    info!("created: {:?}", json!(created).to_string());

    let row = serde_json::from_value::<CollectionResponse>(created).unwrap();

    assert_eq!(row.name, "test");

    // delete row

    let deleted = collections
        .delete(table_name.to_string(), row.id.to_string())
        .await;

    info!("deleted: {deleted:?}");

    assert_eq!(deleted.is_ok(), true);

    let row = collections
        .get(table_name.to_string(), row.id.to_string())
        .await;

    assert_eq!(row.is_ok(), false);
}
