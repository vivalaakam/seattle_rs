use actix_web::{dev::ServiceRequest, error::ErrorForbidden, web, Error};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::debug;

use vivalaakam_seattle_collection::Storage;

use crate::App;

pub async fn validator<T>(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)>
where
    T: Storage + 'static,
{
    debug!("credentials = {:?}", credentials.token());
    let app_data = req.app_data::<web::Data<App<T>>>().unwrap();

    if !app_data.is_valid(credentials.token()) {
        Err((ErrorForbidden(r#"{"error": "forbidden"}"#), req))
    } else {
        Ok(req)
    }
}
