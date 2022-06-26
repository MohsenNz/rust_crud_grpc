use super::dao;
use chrono::Utc;
use log::{debug, info};
use sensor::sensor_server::Sensor;
use sqlx::{Pool, Postgres};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub use sensor::*;

mod sensor {
    tonic::include_proto!("sensor"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct SensorService {
    pool: Pool<Postgres>,
}

impl SensorService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl Sensor for SensorService {
    type PingStream = ReceiverStream<Result<PingResponse, Status>>;

    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<Self::PingStream>, Status> {
        debug!("ping request=\n{:#?}", request);

        let req = request.into_inner();
        let stream_count = req.stream_item_count;

        let (tx, rx) = mpsc::channel(4);

        tokio::spawn(async move {
            for i in 0..stream_count {
                let req = PingResponse {
                    current_time: format!("{}", Utc::now()),
                    message: format!("replay_ping seq={}", i + 1),
                };

                tx.send(Ok(req)).await.unwrap();
            }

            info!(" /// done sending");
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn add(
        &self,
        request: Request<SensorAddRequest>,
    ) -> Result<Response<SensorAddResponse>, Status> {
        let req = request.into_inner();

        let sensor = dao::add_sensor(&self.pool, req.name.clone()).await?;
        let sensor: SensorMessage = sensor.into();

        let sensor = Some(sensor);

        Ok(Response::new(SensorAddResponse { sensor }))
    }

    type GetStream = ReceiverStream<Result<SensorGetResponse, Status>>;

    async fn get(
        &self,
        _request: Request<SensorGetRequest>,
    ) -> Result<Response<Self::GetStream>, Status> {
        unimplemented!()
    }

    async fn delete(
        &self,
        request: Request<SensorDeleteRequest>,
    ) -> Result<Response<SensorDeleteResponse>, Status> {
        let req = request.into_inner();
        let pg_res = dao::delete_sensor(&self.pool, req.id).await?;

        match pg_res.rows_affected() {
            1 => Ok(Response::new(SensorDeleteResponse {})),
            _ => Err(Status::not_found(
                "Correspond sensor not found for deletion",
            )),
        }
    }

    async fn update(
        &self,
        request: Request<SensorUpdateRequest>,
    ) -> Result<Response<SensorUpdateResponse>, Status> {
        let req = request.into_inner();
        let sensor = req.sensor.unwrap();
        let pg_res = dao::update_sensor(&self.pool, &sensor.into()).await?;

        match pg_res.rows_affected() {
            1 => Ok(Response::new(SensorUpdateResponse {})),
            _ => Err(Status::not_found("Correspond sensor not found for update")),
        }
    }
}
