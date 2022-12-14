use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum OddsError {
    // #[error("Eframe error")]
    // EframeError(eframe::EframeError),
    #[error("Failed to read configuration file")]
    ConfigReadError,

    #[error("Failed to parse configuration file")]
    ConfigParseError,

    #[error("Database error")]
    DbError(String),

    #[error("No result found by the given condition")]
    NotFound,

    #[error("No Error has founded")]
    None,
}

impl From<sqlx::Error> for OddsError {
    fn from(err: sqlx::Error) -> Self {
        OddsError::DbError(err.to_string())
    }
}

// impl From<eframe::EframeError> for OddsError {
//     fn from(err: eframe::EframeError) -> Self {
//         OddsError::DbError(sqlx::Error::Database(Box::new(err)))
//     }
// }
