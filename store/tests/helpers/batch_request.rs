use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::helpers::error_response::ErrorResponse;
use crate::helpers::handle_response::handle_response;

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum CollectionAction {
    Create {
        collection: String,
        data: Value,
    },
    Update {
        collection: String,
        identifier: String,
        data: Value,
    },
    Delete {
        collection: String,
        identifier: String,
    },
    Get {
        collection: String,
        identifier: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct BatchRequest {
    pub requests: Vec<CollectionAction>,
}

#[derive(Serialize, Deserialize)]
pub struct BatchResponse {
    pub results: Vec<Value>,
}

pub async fn batch_request<T1>(
    web_app: &T1,
    data: Vec<CollectionAction>,
    secret_code: &String,
) -> Result<BatchResponse, ErrorResponse>
where
    T1: Service<Request, Response = ServiceResponse, Error = Error>,
{
    let req = test::TestRequest::post()
        .uri(&format!("/api/batch"))
        .insert_header(("authorization", format!("Bearer {secret_code}")))
        .set_json(json!({ "requests": data }))
        .to_request();

    let resp = web_app.call(req).await.unwrap();
    handle_response(resp).await
}
