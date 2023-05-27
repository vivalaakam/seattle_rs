use serde_json::Value;

pub enum WhereAttr {
    Eq(Value),
    Ne(Value),
    Gt(Value),
    Lt(Value),
    Gte(Value),
    Lte(Value),
    In(Vec<Value>),
    Nin(Vec<Value>),
}
