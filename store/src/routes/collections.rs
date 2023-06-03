use actix_web::http::header;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use tracing::debug;

use collection::{value_to_string, Storage};

use crate::App;

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

    match app
        .get_collections()
        .get(collection_name, collection_id)
        .await
    {
        Ok(data) => HttpResponse::Ok()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(value_to_string(data)),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
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

    match app
        .get_collections()
        .insert(collection_name, serde_json::from_slice(&data).unwrap())
        .await
    {
        Ok(data) => HttpResponse::Ok()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(value_to_string(data)),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
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

    match app
        .get_collections()
        .delete(collection_name, collection_id)
        .await
    {
        Ok(_) => HttpResponse::Ok()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(()),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
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
    match app
        .get_collections()
        .update(
            collection_name,
            collection_id,
            serde_json::from_slice(&data).unwrap(),
        )
        .await
    {
        Ok(data) => HttpResponse::Ok()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(value_to_string(data)),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
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

    match app.get_collections().list(collection_name, query).await {
        Ok(data) => HttpResponse::Ok()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .body(value_to_string(json!(data))),
        Err(error) => HttpResponse::InternalServerError().json(error),
    }
}
