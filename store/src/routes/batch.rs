use actix_http::header;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use collection::{Storage, value_to_string};

use crate::App;
use crate::collection_action::CollectionAction;

#[derive(Serialize, Deserialize)]
pub struct BatchRequest {
    pub requests: Vec<CollectionAction>,
}

pub async fn batch<T>(
    data: web::Json<BatchRequest>,
    app: web::Data<App<T>>,
) -> HttpResponse
    where
        T: Storage,
{
    let mut results = vec![];

    for row in &data.requests {
        let res = match row.perform(&app).await {
            Ok(data) => data,
            Err(error) => json!(error),
        };

        results.push(res);
    }

    HttpResponse::Ok()
        .insert_header((header::CONTENT_TYPE, "application/json"))
        .body(value_to_string(json!({ "results": results})))
}
