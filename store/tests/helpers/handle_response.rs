use actix_http::body::to_bytes;
use actix_web::dev::ServiceResponse;
use actix_web::http;
use serde::de;
use tracing::debug;

use crate::helpers::error_response::ErrorResponse;

pub async fn handle_response<T: de::DeserializeOwned>(
    response: ServiceResponse,
) -> anyhow::Result<T, ErrorResponse> {
    debug!("response.status() = {}", response.status());
    match response.status() {
        http::StatusCode::OK => {
            let row = to_bytes(response.into_body()).await.unwrap();
            debug!("row = {row:?}");
            Ok(serde_json::from_slice(&row).unwrap())
        }
        http::StatusCode::UNAUTHORIZED
        | http::StatusCode::BAD_REQUEST
        | http::StatusCode::FORBIDDEN => {
            let row = to_bytes(response.into_body()).await.unwrap();
            let err: ErrorResponse = serde_json::from_slice(&row).unwrap();

            Err(err)
        }
        _ => Err(ErrorResponse {
            error: "unknown".to_string(),
        }),
    }
}
