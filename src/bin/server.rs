use ping::{
    ping_server::{Ping, PingServer},
    PingRequest, PingResponse, StatusCode,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod ping {
    tonic::include_proto!("ping");
}
struct PingImpl {}

#[tonic::async_trait]
impl Ping for PingImpl {
    async fn ping(&self, request: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        println!("processing request {:?}", request);
        Ok(Response::new(PingResponse {
            status_code: StatusCode::Success.into(),
        }))
    }
}

#[tokio::main]
async fn main() {
    Server::builder()
        .add_service(PingServer::new(PingImpl {}))
        .serve("[::0]:50001".parse().unwrap())
        .await
        .unwrap()
}
