use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("environment variable not exist")]
    EnvValueNotExist(#[from] std::env::VarError),
    #[error("connection error")]
    ConnectionError(#[from] diesel::result::ConnectionError),
    #[error("connection pool error")]
    ConnectionPoolError(#[from] diesel_async::pooled_connection::deadpool::PoolError),
    #[error("connection pool build error")]
    ConnectionPoolBuildError(#[from] diesel_async::pooled_connection::deadpool::BuildError),
    #[error("db error")]
    DieselError(#[from] diesel::result::Error),
    #[error("tokio join error")]
    TokioError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Unknown(#[from] Box<dyn std::error::Error + std::marker::Send + Sync>),
}

pub type Result<T> = std::result::Result<T, Error>;
