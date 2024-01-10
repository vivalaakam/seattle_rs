use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

use vivalaakam_seattle_collection::Storage;

use crate::validator::validator;

mod batch;
mod collections;

pub fn config<T>(conf: &mut web::ServiceConfig)
where
    T: Storage + 'static,
{
    let scope = web::scope("/api")
        .wrap(HttpAuthentication::bearer(validator::<T>))
        .service(web::resource("/batch").route(web::post().to(batch::batch::<T>)))
        .service(
            web::resource("/collections/{collection}/{object_id}")
                .route(web::get().to(collections::collection_get::<T>))
                .route(web::put().to(collections::collection_update::<T>))
                .route(web::delete().to(collections::collection_delete::<T>)),
        )
        .service(
            web::resource("/collections/{collection}")
                .route(web::get().to(collections::collection_query::<T>))
                .route(web::post().to(collections::collection_create::<T>)),
        );

    conf.service(scope);
}
