use thiserror::Error;

#[derive(Debug, Error)]
pub enum OddsError {
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
