use chrono::{DateTime, Utc};
use serde_json::{json, Map, Value};
use sqlx::{postgres::PgRow, types::Json, Row};

use vivalaakam_seattle_collection::{Collection, FieldType};

pub fn serialize_pg_row(collection: &Collection, row: PgRow) -> Value {
    let mut map = Map::new();

    for field in &collection.fields {
        let v = match field.field_type {
            FieldType::String => match row.get::<Option<String>, _>(field.name.as_str()) {
                Some(v) => Value::String(v),
                None => Value::Null,
            },
            FieldType::Number => match row.get::<Option<f64>, _>(field.name.as_str()) {
                Some(v) => json!(v),
                None => Value::Null,
            },
            FieldType::Boolean => match row.get::<Option<bool>, _>(field.name.as_str()) {
                Some(v) => Value::Bool(v),
                None => Value::Null,
            },
            FieldType::Array | FieldType::Object => {
                match row.get::<Option<Json<Value>>, _>(field.name.as_str()) {
                    Some(v) => v.0,
                    None => Value::Null,
                }
            }
            FieldType::TimeStamp => {
                match row.get::<Option<DateTime<Utc>>, _>(field.name.as_str()) {
                    Some(v) => json!({
                        "__type": "TimeStamp",
                        "value": v.to_rfc3339(),
                    }),
                    None => Value::Null,
                }
            }
        };

        map.insert(field.name.to_string(), v);
    }

    Value::Object(map)
}
