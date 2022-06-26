use anyhow::Result;
use dotenv::dotenv;
use env_logger::Env;
use rust_crud_grpc::config;
use sqlx::postgres::PgPoolOptions;
use tonic::transport::Server;

use rust_crud_grpc::sensor::{sensor_server::SensorServer, SensorService};

#[tokio::main]
async fn main() -> Result<()> {
    // load .env file
    dotenv().ok();

    // init config from environment
    let config = config::Config::from_env().unwrap();

    // init logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // init database connection
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.database_url())
        .await?;

    let addr = config.server_addr().parse()?;
    let sensor = SensorService::new(pool.clone());

    // banner
    println!("=== Rust CRUD GRPC ===");
    println!("Using configuration file from environment varaible or .env");
    println!("Connected to database: {}", config.database_url());
    println!("Listening on: {}\n", config.server_addr());

    // run server
    Server::builder()
        .add_service(SensorServer::new(sensor))
        .serve(addr)
        .await?;

    Ok(())
}
