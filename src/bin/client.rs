use std::env;

use ping::{ping_client::PingClient, PingRequest};
use tonic::Request;

pub mod ping {
    tonic::include_proto!("ping");
}

#[tokio::main]
async fn main() {
    let mut client = PingClient::connect(format!(
        "http://{}:50001",
        env::var("PING_SERVER").unwrap_or("[::0]".into())
    ))
    .await
    .unwrap();

    let response = client
        .ping(Request::new(PingRequest {
            message: "this is a ping request".into(),
        }))
        .await
        .unwrap();

    println!(
        "received response {:?}",
        response.into_inner().status_code()
    );
}
