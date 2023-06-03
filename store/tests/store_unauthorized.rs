use std::env;

use actix_web::{test, web, App as WebApp};
use dotenv::dotenv;
use serde_json::json;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use collection::{Collections, Storage};
use collection_postgres::StorePostgresql;
use store::{routes, App};

use crate::helpers::collection_response::CollectionResponse;
use crate::helpers::create_request::create_request;

mod helpers;

#[tokio::test]
async fn store_unauthorized() {
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

    let fake_code = "fake_code".to_string();

    let row = create_request::<_, CollectionResponse>(
        &web_app,
        &table_name,
        json!({"name": "test","age": 10}),
        &fake_code,
    )
        .await;

    info!("row = {row:?}");

    assert!(row.is_err());
    let row = row.err();

    assert_eq!(format!("{row:?}") , r#"Some(ErrorResponse { error: "forbidden" })"#)
}
