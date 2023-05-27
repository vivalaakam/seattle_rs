use sqlx::{Pool, Postgres};

pub async fn cleanup_table(db: &Pool<Postgres>, table_name: &str) {
    let _ = sqlx::query(format!(r#"DROP TABLE IF EXISTS "{}""#, table_name).as_str())
        .execute(db)
        .await;

    let _ = sqlx::query(r#"DELETE FROM storage_collection_schema WHERE name LIKE $1"#)
        .bind(table_name)
        .execute(db)
        .await;
}
