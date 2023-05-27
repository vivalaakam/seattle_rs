use std::env;

use dotenv::dotenv;
use sqlx::Row;
use tracing_subscriber::filter::LevelFilter;

use collection::{FieldType, StorageCollection, StorageCollectionField, StorageCollectionTrait};
use collection_postgres::StorePostgresql;

#[tokio::test]
async fn store_postgresql() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .with_test_writer()
        .init();

    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");

    let instance = StorePostgresql::new(database_url.as_str()).await;

    let table_name = "CreateSchema1".to_string();

    let _ = sqlx::query(format!(r#"DROP TABLE IF EXISTS "{}""#, table_name.to_string()).as_str())
        .execute(instance.get_pool())
        .await;

    let _ = sqlx::query(r#"DELETE FROM storage_collection_schema WHERE name LIKE $1"#)
        .bind(table_name.to_string())
        .execute(instance.get_pool())
        .await;

    let schema = instance
        .create_collection(StorageCollection {
            name: table_name.to_string(),
            fields: vec![],
            ..Default::default()
        })
        .await;

    assert_eq!(schema.is_ok(), true);

    let schema = schema.unwrap();

    assert_eq!(schema.name, table_name.to_string());
    assert_eq!(schema.fields.len(), 3);

    let string_field_name = "stringField".to_string();

    let schema = instance
        .insert_field_to_collection(
            schema,
            StorageCollectionField {
                name: string_field_name.to_string(),
                field_type: FieldType::String,
            },
        )
        .await;

    assert_eq!(schema.is_ok(), true);
    let schema = schema.unwrap();
    assert_eq!(schema.fields.len(), 4);

    let has_field = schema.fields.clone().into_iter().any(|field| {
        field.name == string_field_name.to_string() && field.field_type == FieldType::String
    });

    assert_eq!(has_field, true);

    let exists = sqlx::query(
        "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = $1 AND column_name = $2;"
    )
        .bind(table_name.to_string())
        .bind(string_field_name.to_string())
        .fetch_optional(instance.get_pool())
        .await.unwrap_or_default();

    assert_eq!(exists.is_some(), true);
    let exists = exists.unwrap();

    assert_eq!(
        exists.get::<String, usize>(0),
        string_field_name.to_string()
    );
    assert_eq!(exists.get::<String, usize>(1), "text".to_string());

    let schema = instance
        .remove_field_from_collection(
            schema,
            StorageCollectionField {
                name: string_field_name.to_string(),
                field_type: FieldType::String,
            },
        )
        .await;

    assert_eq!(schema.is_ok(), true);
    let schema = schema.unwrap();
    assert_eq!(schema.fields.len(), 3);

    let has_field = schema.fields.into_iter().any(|field| {
        field.name == string_field_name.to_string() && field.field_type == FieldType::String
    });

    assert_eq!(has_field, false);

    let exists = sqlx::query(
        "SELECT column_name, data_type FROM information_schema.columns WHERE table_name = $1 AND column_name = $2;"
    )
        .bind(table_name.to_string())
        .bind(string_field_name.to_string())
        .fetch_optional(instance.get_pool())
        .await.unwrap_or_default();

    assert_eq!(exists.is_some(), false);
}
