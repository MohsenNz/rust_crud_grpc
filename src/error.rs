use derive_more::{Display, From};
use log::info;

#[derive(Display, From, Debug)]
pub enum Error {
    CustomErr(String),
    SqlxErr(sqlx::Error),
}

impl std::error::Error for Error {}

impl From<Error> for tonic::Status {
    fn from(err: Error) -> Self {
        match err {
            Error::CustomErr(err) => {
                info!("Error: {err}");
                Self::internal(err)
            }
            Error::SqlxErr(err) => match err {
                sqlx::Error::ColumnNotFound(err) => {
                    info!("sqlx::Error::ColumnNotFound: {err}");
                    Self::not_found(err)
                }
                sqlx::Error::RowNotFound => {
                    info!("sqlx::Error::RowNotFound");
                    Self::not_found("Row not found")
                }
                sqlx::Error::TypeNotFound { type_name } => {
                    info!("sqlx::Error::TypeNotFound: {type_name}");
                    Self::invalid_argument(type_name)
                }
                err => {
                    info!("{err}");
                    Self::internal("Internal Error")
                }
            },
        }
    }
}
