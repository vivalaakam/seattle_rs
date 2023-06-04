use std::env;

use actix_web::{test, web, App as WebApp};
use dotenv::dotenv;
use serde_json::json;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

use collection::{Collections, Storage};
use collection_postgres::StorePostgresql;
use store::{routes, App};

use crate::helpers::batch_request::{batch_request, CollectionAction};
use crate::helpers::collection_response::CollectionResponse;
use crate::helpers::get_request::get_request;

mod helpers;

#[tokio::test]
async fn store_batch() {
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

    let requests = vec![
        CollectionAction::Create {
            collection: table_name.to_string(),
            data: json!({"name": "test1","age": 30}),
        },
        CollectionAction::Create {
            collection: table_name.to_string(),
            data: json!({"name": "test2","age": 31}),
        },
        CollectionAction::Create {
            collection: table_name.to_string(),
            data: json!({"name": "test3","age": 32}),
        },
    ];

    let results = batch_request(&web_app, requests, &secret_code).await;

    assert!(results.is_ok());
    let rows = results.unwrap();

    let first_row = match rows
        .results
        .get(0)
        .map(|v| serde_json::from_value::<CollectionResponse>(v.clone()))
    {
        Some(Ok(row)) => {
            assert_eq!(row.name, "test1");
            assert_eq!(row.age, 30);
            row.id
        }
        _ => {
            assert!(false);
            "".to_string()
        }
    };

    let second_row = match rows
        .results
        .get(1)
        .map(|v| serde_json::from_value::<CollectionResponse>(v.clone()))
    {
        Some(Ok(row)) => {
            assert_eq!(row.name, "test2");
            assert_eq!(row.age, 31);
            row.id
        }
        _ => {
            assert!(false);
            "".to_string()
        }
    };

    let third_row = match rows
        .results
        .get(2)
        .map(|v| serde_json::from_value::<CollectionResponse>(v.clone()))
    {
        Some(Ok(row)) => {
            assert_eq!(row.name, "test3");
            assert_eq!(row.age, 32);
            row.id
        }
        _ => {
            assert!(false);
            "".to_string()
        }
    };

    let check_row =
        get_request::<_, CollectionResponse>(&web_app, &table_name, &first_row, &secret_code).await;

    match check_row {
        Ok(row) => {
            assert_eq!(row.name, "test1");
            assert_eq!(row.age, 30);
        }
        Err(err) => {
            info!("err = {err:?}");
            assert!(false);
        }
    }

    let check_row_2 =
        get_request::<_, CollectionResponse>(&web_app, &table_name, &second_row, &secret_code)
            .await;

    match check_row_2 {
        Ok(row) => {
            assert_eq!(row.name, "test2");
            assert_eq!(row.age, 31);
        }
        Err(err) => {
            info!("err = {err:?}");
            assert!(false);
        }
    }

    let requests = vec![
        CollectionAction::Create {
            collection: table_name.to_string(),
            data: json!({"name": "test4","age": 34}),
        },
        CollectionAction::Update {
            collection: table_name.to_string(),
            identifier: second_row,
            data: json!({"name": "test5","age": 35}),
        },
        CollectionAction::Delete {
            collection: table_name.to_string(),
            identifier: third_row,
        },
        CollectionAction::Get {
            collection: table_name.to_string(),
            identifier: first_row,
        },
    ];

    let results = batch_request(&web_app, requests, &secret_code).await;

    assert!(results.is_ok());
    let rows = results.unwrap();

    match rows
        .results
        .get(0)
        .map(|v| serde_json::from_value::<CollectionResponse>(v.clone()))
    {
        Some(Ok(row)) => {
            assert_eq!(row.name, "test4");
            assert_eq!(row.age, 34);
        }
        _ => assert!(false),
    }

    match rows
        .results
        .get(1)
        .map(|v| serde_json::from_value::<CollectionResponse>(v.clone()))
    {
        Some(Ok(row)) => {
            assert_eq!(row.name, "test5");
            assert_eq!(row.age, 35);
        }
        _ => assert!(false),
    }

    match rows
        .results
        .get(3)
        .map(|v| serde_json::from_value::<CollectionResponse>(v.clone()))
    {
        Some(Ok(row)) => {
            assert_eq!(row.name, "test1");
            assert_eq!(row.age, 30);
        }
        _ => assert!(false),
    }
}
