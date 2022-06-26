use anyhow::Result;
use rust_crud_grpc::sensor::sensor_client::SensorClient;
use rust_crud_grpc::sensor::PingRequest;
use std::error::Error;
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = SensorClient::connect("http://[::1]:50051").await?;

    ping(&mut client).await.unwrap();

    Ok(())
}

async fn ping(client: &mut SensorClient<Channel>) -> Result<(), Box<dyn Error>> {
    let req = tonic::Request::new(PingRequest {
        stream_item_count: 3,
    });

    let mut stream = client.ping(req).await?.into_inner();

    while let Some(res) = stream.message().await? {
        println!("{:?}", res.message);
    }

    Ok(())
}
