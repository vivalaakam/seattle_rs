use std::env;

use actix_web::{App as WebApp, test, web};
use actix_web::body::to_bytes;
use actix_web::dev::Service;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use collection::{Collections, Storage};
use collection_postgres::StorePostgresql;
use store::{App, routes};

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

    let _ = instance.remove_collection(table_name.to_string()).await;

    let collections = Collections::new(instance).await;

    let app = App::new(collections);

    let web_app = test::init_service(
        WebApp::new()
            .app_data(web::Data::new(app.clone()))
            .configure(routes::config::<StorePostgresql>),
    )
        .await;

    let req = test::TestRequest::post()
        .uri(&format!("/api/collections/{table_name}"))
        .set_json(json!({"name": "test","age": 10,}))
        .to_request();

    let resp = web_app.call(req).await.unwrap();

    let row = to_bytes(resp.into_body()).await.unwrap();
    info!("row: {:?}", row);

    let row = serde_json::from_slice::<CollectionResponse>(&row).unwrap();

    assert_eq!(row.name, "test");
    assert_eq!(row.age, 10);

    let check_req = test::TestRequest::get()
        .uri(&format!("/api/collections/{table_name}/{id}", id = row.id))
        .set_json(json!({"name": "test","age": 10,}))
        .to_request();

    let check_resp = web_app.call(check_req).await.unwrap();

    let check_row = to_bytes(check_resp.into_body()).await.unwrap();
    info!("check_row: {:?}", check_row);

    let check_row = serde_json::from_slice::<CollectionResponse>(&check_row).unwrap();


    let check_req_2 = test::TestRequest::get()
        .uri(&format!("/api/collections/{table_name}/{id}", id = row.id))
        .set_json(json!({"name": "test","age": 10,}))
        .to_request();

    let check_resp_2 = web_app.call(check_req_2).await.unwrap();

    let check_row_2 = to_bytes(check_resp_2.into_body()).await.unwrap();
    info!("check_row: {:?}", check_row_2);

    let check_row_2 = serde_json::from_slice::<CollectionResponse>(&check_row_2).unwrap();

    assert_eq!(check_row_2, check_row);
}
