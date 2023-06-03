use std::env;

use actix_web::{App as WebApp, test, web};
use dotenv::dotenv;
use serde_json::json;
use tracing_subscriber::filter::LevelFilter;

use collection::{Collections, Storage};
use collection_postgres::StorePostgresql;
use store::{App, routes};

use crate::helpers::collection_response::CollectionResponse;
use crate::helpers::create_request::create_request;
use crate::helpers::get_request::get_request;

mod helpers;

#[tokio::test]
async fn store_get() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();

    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");

    let instance = StorePostgresql::new(database_url.as_str()).await;

    let table_name = "Collection1".to_string();

    let _ = instance.remove_collection(table_name.to_string()).await;

    let collections = Collections::new(instance).await;

    let secret_code = env::var("SECRET_CODE").expect("SECRET_CODE must be set");

    let app = App::new(collections, secret_code.to_string());

    let web_app = test::init_service(
        WebApp::new()
            .app_data(web::Data::new(app.clone()))
            .configure(routes::config::<StorePostgresql>),
    )
        .await;

    let row: CollectionResponse =
        create_request(&web_app, &table_name, json!({"name": "test","age": 10}), &secret_code).await;

    assert_eq!(row.name, "test");
    assert_eq!(row.age, 10);

    let check_row: CollectionResponse = get_request(&web_app, &table_name, &row.id, &secret_code).await;

    let check_row_2: CollectionResponse = get_request(&web_app, &table_name, &row.id, &secret_code).await;

    assert_eq!(check_row_2, check_row);
}
