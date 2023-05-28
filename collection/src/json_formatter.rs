use std::io::Write;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct JsonFormatter {}

impl serde_json::ser::Formatter for JsonFormatter {
    #[inline]
    fn write_f32<W>(&mut self, writer: &mut W, value: f32) -> std::io::Result<()> where W: ?Sized + Write {
        let mut v = value.to_string();
        if v.ends_with(".0") {
            v.truncate(v.len() - 2)
        }

        writer.write_all(v.as_bytes())
    }

    #[inline]
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> std::io::Result<()> where W: ?Sized + Write {
        let mut v = value.to_string();
        if v.ends_with(".0") {
            v.truncate(v.len() - 2)
        }

        writer.write_all(v.as_bytes())
    }
}

impl JsonFormatter {
    pub fn new() -> Self {
        JsonFormatter {}
    }
}

pub fn serialize_value(value: Value) -> String {
    let mut writer = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(&mut writer, JsonFormatter::new());
    value.serialize(&mut ser).unwrap();
    unsafe {
        String::from_utf8_unchecked(writer)
    }
}


#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::serialize_value;

    #[test]
    fn serialize_value_test() {
        let data = json!({
            "a": "string",
            "b": 1.2,
            "c": 1.0,
            "d": 2,
            "e": true,
            "f": {
                "g": ["i", 1.0, 1.2]
            }
        });
        let item = serialize_value(data);

        assert_eq!(
            item,
            r#"{"a":"string","b":1.2,"c":1,"d":2,"e":true,"f":{"g":["i",1,1.2]}}"#
        );
    }
}

