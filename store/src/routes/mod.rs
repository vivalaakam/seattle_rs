use actix_web::web;

use collection::Storage;

mod collections;

pub fn config<T>(conf: &mut web::ServiceConfig) where T: Storage + 'static {
    let scope = web::scope("/api")
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
