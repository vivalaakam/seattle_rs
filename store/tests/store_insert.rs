use std::env;

use actix_web::{test, web, App as WebApp};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::filter::LevelFilter;

use store::{routes, App};
use vivalaakam_seattle_collection::{Collection, Collections, Storage};
use vivalaakam_seattle_collection_postgres::StorePostgresql;

use crate::helpers::create_request::create_request;
use crate::helpers::get_request::get_request;

mod helpers;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CollectionResponse {
    id: String,
    name: String,
    age: i32,
}

#[tokio::test]
async fn store_insert() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();

    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");

    let instance = StorePostgresql::new(database_url.as_str()).await;

    let table_name = "Collection1".to_string();

    let _ = instance
        .remove_collection(&Collection {
            name: table_name.to_string(),
            ..Default::default()
        })
        .await;

    let collections = Collections::new(instance).await;

    let secret_code = env::var("SECRET_CODE").expect("SECRET_CODE must be set");

    let app = App::new(collections, secret_code.to_string());

    let web_app = test::init_service(
        WebApp::new()
            .app_data(web::Data::new(app.clone()))
            .configure(routes::config::<StorePostgresql>),
    )
    .await;

    let row = create_request::<_, CollectionResponse>(
        &web_app,
        &table_name,
        json!({"name": "test","age": 10}),
        &secret_code,
    )
    .await;

    assert!(row.is_ok());

    let row = row.unwrap();

    assert_eq!(row.name, "test");
    assert_eq!(row.age, 10);

    let check_row =
        get_request::<_, CollectionResponse>(&web_app, &table_name, &row.id, &secret_code).await;

    assert!(check_row.is_ok());

    let check_row = check_row.unwrap();

    assert_eq!(row, check_row);
}
