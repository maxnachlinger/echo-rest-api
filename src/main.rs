mod routes;
mod settings;
mod grpc;

use crate::routes::{get_echo, openapi, post_echo};
use crate::settings::{get_settings, AppOptions};
use actix_web::{middleware, App, HttpServer};
use crate::grpc::setup_grpc;

#[actix_web::main]
async fn main() {
    let settings = get_settings();
    let AppOptions {
        host,
        port,
        socket_address,
    } = settings.app;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let parsed_socket_address = socket_address.parse().expect("Cannot parse socket address");

    tokio::spawn(async move {
        setup_grpc()
            .serve(parsed_socket_address)
            .await
            .expect("Cannot start gRPC server")
    });

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(get_echo)
            .service(post_echo)
            .service(openapi)
    })
        .bind((host, port))
        .expect("Cannot start REST server")
        .run()
        .await
        .expect("Cannot start REST server")
}
