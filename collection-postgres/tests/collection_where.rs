use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing_subscriber::filter::LevelFilter;

use collection::{value_to_string, Collections};
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize)]
struct CollectionResponse {
    id: String,
    name: String,
    age: i32,
}

#[tokio::test]
async fn collection_where() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();

    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");

    let instance = StorePostgresql::new(database_url.as_str()).await;

    let table_name = "CollectionWhereEq".to_string();

    cleanup_table(instance.get_pool(), &table_name).await;

    let mut collections = Collections::new(instance).await;

    let mut rows = vec![];

    for i in 0..5 {
        let result = collections
            .insert(
                table_name.to_string(),
                json!({ "name": format!("test_{}", i), "age": 10 + i }),
            )
            .await;

        assert_eq!(result.is_ok(), true);

        let result = value_to_string(result.unwrap());

        let row = serde_json::from_str::<CollectionResponse>(&result).unwrap();

        rows.push(row);
    }

    let test_row_0 = rows.get(0).unwrap();
    let test_row_1 = rows.get(1).unwrap();

    // test eq

    let query_string = format!(r#"{{"id": "{}" }}"#, test_row_0.id);

    let result_eq = collections
        .list(
            table_name.to_string(),
            serde_json::from_str(query_string.as_str()).unwrap(),
        )
        .await;

    assert_eq!(result_eq.is_ok(), true);

    let result_eq = result_eq.unwrap();
    assert_eq!(result_eq.len(), 1);

    let rows_eq = result_eq
        .into_iter()
        .map(|row| row.get("id").unwrap().as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    assert_eq!(rows_eq.get(0).unwrap(), &test_row_0.id);

    // test neq

    let query_string = format!(r#"{{"id": {{"$ne": "{}"}} }}"#, test_row_0.id);

    let result_ne = collections
        .list(
            table_name.to_string(),
            serde_json::from_str(query_string.as_str()).unwrap(),
        )
        .await;

    assert_eq!(result_ne.is_ok(), true);

    let result_ne = result_ne.unwrap();
    assert_eq!(result_ne.len(), 4);

    let rows_ne = result_ne
        .into_iter()
        .map(|row| row.get("id").unwrap().as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    assert_eq!(rows_ne.contains(&test_row_0.id), false);

    // test in

    let query_string = format!(
        r#"{{"id": {{"$in": ["{}", "{}", "{}"] }} }}"#,
        test_row_0.id, test_row_1.id, "some value"
    );

    let result_in = collections
        .list(
            table_name.to_string(),
            serde_json::from_str(query_string.as_str()).unwrap(),
        )
        .await;

    assert_eq!(result_in.is_ok(), true);

    let result_in = result_in.unwrap();
    assert_eq!(result_in.len(), 2);

    let rows_in = result_in
        .into_iter()
        .map(|row| row.get("id").unwrap().as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    assert_eq!(rows_in.contains(&test_row_0.id), true);
    assert_eq!(rows_in.contains(&test_row_1.id), true);

    // test nin

    let query_string = format!(
        r#"{{"id": {{"$nin": ["{}", "{}", "{}"] }} }}"#,
        test_row_0.id, test_row_1.id, "some value"
    );

    let result_nin = collections
        .list(
            table_name.to_string(),
            serde_json::from_str(query_string.as_str()).unwrap(),
        )
        .await;

    assert_eq!(result_nin.is_ok(), true);

    let result_nin = result_nin.unwrap();
    assert_eq!(result_nin.len(), 3);

    let rows_nin = result_nin
        .into_iter()
        .map(|row| row.get("id").unwrap().as_str().unwrap().to_string())
        .collect::<Vec<String>>();

    assert_eq!(rows_nin.contains(&test_row_0.id), false);
    assert_eq!(rows_nin.contains(&test_row_1.id), false);
}
