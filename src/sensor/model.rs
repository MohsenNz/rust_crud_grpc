use super::SensorMessage;

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct SensorModel {
    pub id: i32,
    pub name: String,
}

impl From<SensorMessage> for SensorModel {
    fn from(sensor: SensorMessage) -> Self {
        Self {
            id: sensor.id,
            name: sensor.name,
        }
    }
}

impl From<SensorModel> for SensorMessage {
    fn from(sensor: SensorModel) -> Self {
        Self {
            id: sensor.id,
            name: sensor.name,
        }
    }
}
