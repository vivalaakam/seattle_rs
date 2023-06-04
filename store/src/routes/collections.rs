use actix_web::http::header;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tracing::debug;

use collection::{value_to_string, CollectionError, Storage};

use crate::collection_action::CollectionAction;
use crate::App;

fn perform_result<T>(result: Result<T, CollectionError>) -> HttpResponse
where
    T: Serialize,
{
    match result {
        Ok(data) => HttpResponse::Ok()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(value_to_string(data)),
        Err(CollectionError::CollectionNotFound { collection }) => {
            HttpResponse::NotFound().json(collection)
        }
        Err(error) => HttpResponse::BadRequest().json(error),
    }
}

#[derive(Debug, Deserialize)]
pub struct CollectionQuery {
    #[serde(rename = "where")]
    pub where_param: Option<String>,
}

pub async fn collection_get<T>(
    path: web::Path<(String, String)>,
    app: web::Data<App<T>>,
) -> HttpResponse
where
    T: Storage,
{
    debug!("collection_get {path:?}");
    let (collection_name, collection_id) = path.into_inner();

    let action = CollectionAction::Get {
        collection: collection_name,
        identifier: collection_id,
    };

    perform_result(action.perform(&app).await)
}

pub async fn collection_create<T>(
    path: web::Path<String>,
    data: web::Bytes,
    app: web::Data<App<T>>,
) -> HttpResponse
where
    T: Storage,
{
    debug!("collection_create {path:?}");
    let collection_name = path.into_inner();

    let action = CollectionAction::Create {
        collection: collection_name.to_string(),
        data: serde_json::from_slice(&data).unwrap(),
    };

    perform_result(action.perform(&app).await)
}

pub async fn collection_delete<T>(
    path: web::Path<(String, String)>,
    app: web::Data<App<T>>,
) -> HttpResponse
where
    T: Storage,
{
    debug!("collection_delete {path:?}");
    let (collection_name, collection_id) = path.into_inner();

    let action = CollectionAction::Delete {
        collection: collection_name,
        identifier: collection_id,
    };

    perform_result(action.perform(&app).await)
}

pub async fn collection_update<T>(
    path: web::Path<(String, String)>,
    data: web::Bytes,
    app: web::Data<App<T>>,
) -> HttpResponse
where
    T: Storage,
{
    debug!("collection_update {path:?}");
    let (collection_name, collection_id) = path.into_inner();

    let action = CollectionAction::Update {
        collection: collection_name,
        identifier: collection_id,
        data: serde_json::from_slice(&data).unwrap(),
    };

    perform_result(action.perform(&app).await)
}

pub async fn collection_query<T>(
    path: web::Path<String>,
    query: web::Query<CollectionQuery>,
    app: web::Data<App<T>>,
) -> HttpResponse
where
    T: Storage,
{
    debug!("collection_query {path:?} {query:?}");
    let collection_name = path.into_inner();

    let query = query
        .where_param
        .as_ref()
        .map(|v| serde_json::from_str(v.as_str()).unwrap())
        .unwrap_or(Value::Object(Map::new()));

    let result = app.get_collections().list(collection_name, query).await;

    perform_result(result)
}
