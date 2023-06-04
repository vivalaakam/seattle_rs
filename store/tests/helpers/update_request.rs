use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{test, Error};
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::helpers::error_response::ErrorResponse;
use crate::helpers::handle_response::handle_response;

pub async fn update_request<T1, T2>(
    web_app: &T1,
    collection_name: &String,
    collection_id: &String,
    data: Value,
    secret_code: &String,
) -> Result<T2, ErrorResponse>
where
    T1: Service<Request, Response = ServiceResponse, Error = Error>,
    T2: DeserializeOwned,
{
    let req = test::TestRequest::put()
        .uri(&format!(
            "/api/collections/{collection_name}/{collection_id}"
        ))
        .insert_header(("authorization", format!("Bearer {secret_code}")))
        .set_json(data)
        .to_request();

    let resp = web_app.call(req).await.unwrap();
    handle_response(resp).await
}
