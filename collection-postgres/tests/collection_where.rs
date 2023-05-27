use std::collections::HashMap;
use std::env;

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing_subscriber::filter::LevelFilter;

use collection::{
    FieldType, StorageCollection, StorageCollectionField, StorageCollectionTrait, WhereAttr,
};
use collection_postgres::StorePostgresql;
use helpers::cleanup_table::cleanup_table;

mod helpers;

#[derive(Serialize, Deserialize)]
struct CollectionResponse {
    id: String,
    name: String,
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

    let schema = instance
        .create_collection(StorageCollection {
            name: table_name.to_string(),
            fields: vec![StorageCollectionField {
                name: "name".to_string(),
                field_type: FieldType::String,
            }],
            ..Default::default()
        })
        .await;

    assert_eq!(schema.is_ok(), true);

    let mut rows = vec![];

    for i in 0..5 {
        let result = instance
            .insert_data_into_collection(
                table_name.to_string(),
                json!({ "name": format!("test_{}", i) }),
            )
            .await;

        assert_eq!(result.is_ok(), true);

        let row = serde_json::from_value::<CollectionResponse>(result.unwrap()).unwrap();
        rows.push(row);
    }

    let test_row_0 = rows.get(0).unwrap();
    let test_row_1 = rows.get(1).unwrap();

    // test eq

    let where_clause_eq = HashMap::from_iter([(
        "id".to_string(),
        WhereAttr::Eq(Value::String(test_row_0.id.to_string())),
    )]);

    let result_eq = instance
        .list_data_from_collection(table_name.to_string(), where_clause_eq)
        .await;

    assert_eq!(result_eq.is_ok(), true);

    let result_eq = result_eq.unwrap();
    assert_eq!(result_eq.len(), 1);

    let rows_eq = result_eq
        .into_iter()
        .map(|row| serde_json::from_value::<CollectionResponse>(row).unwrap().id)
        .collect::<Vec<_>>();

    assert_eq!(rows_eq.get(0).unwrap(), &test_row_0.id);

    // test neq

    let where_clause_ne = HashMap::from_iter([(
        "id".to_string(),
        WhereAttr::Ne(Value::String(test_row_0.id.to_string())),
    )]);

    let result_ne = instance
        .list_data_from_collection(table_name.to_string(), where_clause_ne)
        .await;

    assert_eq!(result_ne.is_ok(), true);

    let result_ne = result_ne.unwrap();
    assert_eq!(result_ne.len(), 4);

    let rows_ne = result_ne
        .into_iter()
        .map(|row| serde_json::from_value::<CollectionResponse>(row).unwrap().id)
        .collect::<Vec<_>>();

    assert_eq!(rows_ne.contains(&test_row_0.id), false);

    // test in

    let where_clause_in = HashMap::from_iter([(
        "id".to_string(),
        WhereAttr::In( vec![ Value::String(test_row_0.id.to_string()), Value::String(test_row_1.id.to_string()), Value::String("some value".to_string())  ] ),
    )]);

    let result_in = instance
        .list_data_from_collection(table_name.to_string(), where_clause_in)
        .await;

    assert_eq!(result_in.is_ok(), true);

    let result_in = result_in.unwrap();
    assert_eq!(result_in.len(), 2);

    let rows_in = result_in
        .into_iter()
        .map(|row| serde_json::from_value::<CollectionResponse>(row).unwrap().id)
        .collect::<Vec<_>>();

    assert_eq!(rows_in.contains(&test_row_0.id), true);
    assert_eq!(rows_in.contains(&test_row_1.id), true);


    // test nin

    let where_clause_nin = HashMap::from_iter([(
        "id".to_string(),
        WhereAttr::Nin( vec![ Value::String(test_row_0.id.to_string()), Value::String(test_row_1.id.to_string()), Value::String("some value".to_string())  ] ),
    )]);

    let result_nin = instance
        .list_data_from_collection(table_name.to_string(), where_clause_nin)
        .await;

    assert_eq!(result_nin.is_ok(), true);

    let result_nin = result_nin.unwrap();
    assert_eq!(result_nin.len(), 3);

    let rows_nin = result_nin
        .into_iter()
        .map(|row| serde_json::from_value::<CollectionResponse>(row).unwrap().id)
        .collect::<Vec<_>>();

    assert_eq!(rows_nin.contains(&test_row_0.id), false);
    assert_eq!(rows_nin.contains(&test_row_1.id), false);
}
