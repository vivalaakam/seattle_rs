use actix_http::body::to_bytes;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::{http, test, Error};
use serde::de::DeserializeOwned;
use serde_json::Value;
use tracing::info;

pub async fn delete_request<T1>(web_app: &T1, collection_name: &String, collection_id: &String) -> ()
where
    T1: Service<Request, Response = ServiceResponse, Error = Error>,
{
    let req = test::TestRequest::delete()
        .uri(&format!("/api/collections/{collection_name}/{collection_id}"))
        .to_request();

    let resp = web_app.call(req).await.unwrap();
    assert_eq!(resp.status(), http::StatusCode::OK);

    ()
}
