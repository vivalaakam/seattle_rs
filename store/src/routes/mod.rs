use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use collection::Storage;

use crate::validator::validator;

mod collections;

pub fn config<T>(conf: &mut web::ServiceConfig)
where
    T: Storage + 'static,
{
    let scope = web::scope("/api")
        .wrap(HttpAuthentication::bearer(validator::<T>))
        .service(
            web::resource("/collections/{collection}/{object_id}")
                .wrap(HttpAuthentication::bearer(validator::<T>))
                .route(web::get().to(collections::collection_get::<T>))
                .route(web::put().to(collections::collection_update::<T>))
                .route(web::delete().to(collections::collection_delete::<T>)),
        )
        .service(
            web::resource("/collections/{collection}")
                .wrap(HttpAuthentication::bearer(validator::<T>))
                .route(web::get().to(collections::collection_query::<T>))
                .route(web::post().to(collections::collection_create::<T>)),
        );

    conf.service(scope);
}
