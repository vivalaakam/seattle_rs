use std::env;
use std::str::FromStr;

use actix_web::{web, App as WebApp, HttpServer};
use dotenv::dotenv;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::format;
use collection::Collections;
use collection_postgres::StorePostgresql;

use store::{routes, App};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let level = LevelFilter::from_str(env::var("LOG_LEVEL").unwrap_or("info".to_string()).as_str())
        .unwrap_or(LevelFilter::INFO);

    tracing_subscriber::fmt()
        .with_max_level(level)
        .event_format(
            format()
                .compact()
                .with_ansi(env::var("PRETTY_LOG").unwrap_or("false".to_string()) == "true")
                .without_time(),
        )
        .init();

    let _guard = sentry::init((
        env::var("SENTRY").unwrap_or_default(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));


    let database_url =
        env::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set");

    let instance = StorePostgresql::new(database_url.as_str()).await;

    let collections = Collections::new(instance).await;

    let app = App::new(collections);

    let app_port = env::var("PORT").unwrap_or(String::from("8080"));

    HttpServer::new(move || {
        WebApp::new()
            .app_data(web::Data::new(app.clone()))
            .configure(routes::config::<StorePostgresql>)
    })
    .bind(format!("0.0.0.0:{app_port}"))
    .expect("panic")
    .run()
    .await
}
