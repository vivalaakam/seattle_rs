use actix_http::body::to_bytes;
use actix_http::Request;
use actix_web::{Error, http, test};
use actix_web::dev::{Service, ServiceResponse};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::info;

pub async fn update_request<T1, T2>(web_app: &T1, collection_name: &String, collection_id: &String, data: Value) -> T2 where
    T1: Service<Request, Response=ServiceResponse, Error=Error>,
    T2: DeserializeOwned, {
    let req = test::TestRequest::put()
        .uri(&format!("/api/collections/{collection_name}/{collection_id}"))
        .set_json(data)
        .to_request();

    let resp = web_app.call(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    let row = to_bytes(resp.into_body()).await.unwrap();
    info!("row: {:?}", row);

    serde_json::from_slice(&row).unwrap()
}