mod settings;
mod routes;

use crate::settings::{get_settings, AppOptions};
use actix_web::{middleware, App, HttpServer};
use crate::routes::{get_echo, openapi, post_echo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = get_settings();
    let AppOptions { host, port } = settings.app;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Listening at {}:{}", &host, &port);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_echo)
            .service(post_echo)
            .service(openapi)
    })
    .bind((host, port))?
    .run()
    .await
}
