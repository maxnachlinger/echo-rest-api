mod routes;
mod settings;

use crate::routes::{get_echo, openapi, post_echo};
use crate::settings::{get_settings, AppOptions};
use actix_web::{middleware, App, HttpServer};
use echo::echo_server::{Echo, EchoServer};
use echo::MessageToEcho;
use tonic::{transport::Server, Request, Response, Status};

pub mod echo {
    tonic::include_proto!("echo");
}

#[derive(Debug, Default)]
pub struct EchoService {}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn echo_message(
        &self,
        request: Request<MessageToEcho>,
    ) -> Result<Response<MessageToEcho>, Status> {
        Ok(Response::new(MessageToEcho {
            message: request.into_inner().message,
        }))
    }
}

#[actix_web::main]
async fn main() {
    let settings = get_settings();
    let AppOptions {
        host,
        port,
        socket_address,
    } = settings.app;

    let parsed_socket_address = socket_address.parse().expect("Cannot parse socket address");
    let echo_service = EchoService::default();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    tokio::spawn(async move {
        Server::builder()
            .add_service(EchoServer::new(echo_service))
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
