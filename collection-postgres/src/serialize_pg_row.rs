use chrono::{DateTime, Utc};
use serde_json::{json, Map, Value};
use sqlx::{postgres::PgRow, Row, types::Json};

use collection::{FieldType, StorageCollection};

pub fn serialize_pg_row(collection: &StorageCollection, row: PgRow) -> Value {
    let mut map = Map::new();

    for field in &collection.fields {
        let v = match field.field_type {
            FieldType::String => match row.get::<Option<String>, _>(field.name.as_str()) {
                Some(v) => v.into(),
                None => Value::Null,
            },
            FieldType::Number => match row.get::<Option<f64>, _>(field.name.as_str()) {
                Some(v) => v.into(),
                None => Value::Null,
            },
            FieldType::Boolean => match row.get::<Option<bool>, _>(field.name.as_str()) {
                Some(v) => v.into(),
                None => Value::Null,
            },
            FieldType::Array => match row.get::<Option<Json<Value>>, _>(field.name.as_str()) {
                Some(v) => v.0.into(),
                None => Value::Null,
            },
            FieldType::Object => match row.get::<Option<Json<Value>>, _>(field.name.as_str()) {
                Some(v) => v.0.into(),
                None => Value::Null,
            },
            FieldType::TimeStamp => match row.get::<Option<DateTime<Utc>>, _>(field.name.as_str()) {
                Some(v) => json!({
                    "__type": "TimeStamp",
                    "value": v.to_rfc3339(),
                }),
                None => Value::Null,
            },
        };

        map.insert(field.name.to_string(), v.into());
    }

    Value::Object(map)
}
