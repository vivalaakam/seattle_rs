use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Default)]
pub struct Where {
    #[serde(rename = "$eq")]
    pub eq: Option<Value>,
    #[serde(rename = "$ne")]
    pub ne: Option<Value>,
    #[serde(rename = "$gt")]
    pub gt: Option<Value>,
    #[serde(rename = "$lt")]
    pub lt: Option<Value>,
    #[serde(rename = "$gte")]
    pub gte: Option<Value>,
    #[serde(rename = "$lte")]
    pub lte: Option<Value>,
    #[serde(rename = "$in")]
    pub in_: Option<Vec<Value>>,
    #[serde(rename = "$nin")]
    pub nin: Option<Vec<Value>>,
}
