use actix_http::body::to_bytes;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{http, test, Error};

use crate::helpers::error_response::ErrorResponse;

pub async fn delete_request<T1>(
    web_app: &T1,
    collection_name: &String,
    collection_id: &String,
    secret_code: &String,
) -> anyhow::Result<(), ErrorResponse>
where
    T1: Service<Request, Response = ServiceResponse, Error = Error>,
{
    let req = test::TestRequest::delete()
        .uri(&format!(
            "/api/collections/{collection_name}/{collection_id}"
        ))
        .insert_header(("authorization", format!("Bearer {secret_code}")))
        .to_request();

    let response = web_app.call(req).await.unwrap();

    match response.status() {
        http::StatusCode::OK => Ok(()),
        http::StatusCode::UNAUTHORIZED | http::StatusCode::BAD_REQUEST => {
            let row = to_bytes(response.into_body()).await.unwrap();
            let err: ErrorResponse = serde_json::from_slice(&row).unwrap();

            Err(err)
        }
        _ => Err(ErrorResponse {
            error: "unknown".to_string(),
        }),
    }
}
