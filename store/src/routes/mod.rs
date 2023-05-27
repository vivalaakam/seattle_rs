use actix_web::web;

mod collections;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(collections::collection_get)
        .service(collections::collection_create)
        .service(collections::collection_update)
        .service(collections::collection_delete)
        .service(collections::collection_query);

    conf.service(scope);
}
