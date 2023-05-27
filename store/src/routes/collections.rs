use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use tracing::{event, Level};

use crate::App;

#[get("/classes/{collection}/{object_id}")]
pub async fn collection_get(
    path: web::Path<(String, String)>,
    _app: web::Data<App>,
) -> HttpResponse {
    event!(Level::DEBUG, "collection_get {path:?}");
    HttpResponse::Ok().finish()
}

#[post("/classes/{collection}")]
pub async fn collection_create(_path: web::Path<String>, _app: web::Data<App>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[delete("/classes/{collection}/{object_id}")]
pub async fn collection_delete(
    _path: web::Path<(String, String)>,
    _app: web::Data<App>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[put("/classes/{collection}/{object_id}")]
pub async fn collection_update(
    _path: web::Path<(String, String)>,
    _app: web::Data<App>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/classes/{collection}/{object_id}")]
pub async fn collection_query(
    _path: web::Path<(String, String)>,
    _app: web::Data<App>,
) -> HttpResponse {
    HttpResponse::Ok().finish()
}
