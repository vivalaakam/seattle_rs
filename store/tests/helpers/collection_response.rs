use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CollectionResponse {
    pub id: String,
    pub name: String,
    pub age: i32,
}
