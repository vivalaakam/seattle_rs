use serde_json::Value;
use sqlx::Arguments;
use sqlx::postgres::PgArguments;

use collection::{FieldType, StorageCollectionField};

pub fn add_value_into_args(field: &StorageCollectionField, value: &Value, args: &mut PgArguments) {
    match field.field_type {
        FieldType::String | FieldType::Array | FieldType::Object => {
            args.add(value.as_str());
        }
        FieldType::Number => {
            args.add(value.as_f64());
        }
        FieldType::Boolean => {
            args.add(value.as_bool());
        }
        FieldType::TimeStamp => {
            args.add(value.as_i64());
        }
    };
}
