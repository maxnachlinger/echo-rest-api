use echo::echo_server::{Echo, EchoServer};
use echo::MessageToEcho;
use tonic::transport::server::Router;
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

pub fn setup_grpc() -> Router {
    Server::builder().add_service(EchoServer::new(EchoService::default()))
}
