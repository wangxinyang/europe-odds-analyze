use thiserror::Error;

#[derive(Debug, Error)]
pub enum OddsError {
    // #[error("Eframe error")]
    // EframeError(eframe::EframeError),
    #[error("Database error")]
    DbError(sqlx::Error),

    #[error("No result found by the given condition")]
    NotFound,
}

impl From<sqlx::Error> for OddsError {
    fn from(err: sqlx::Error) -> Self {
        OddsError::DbError(err)
    }
}

// impl From<eframe::EframeError> for OddsError {
//     fn from(err: eframe::EframeError) -> Self {
//         OddsError::DbError(sqlx::Error::Database(Box::new(err)))
//     }
// }
