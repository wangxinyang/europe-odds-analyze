use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, FromRow, Row};

#[derive(Debug)]
pub struct BookMaker {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl BookMaker {
    pub fn new_pending(
        name: impl Into<String>,
        url: Option<impl Into<String>>,
        note: Option<impl Into<String>>,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: 0,
            name: name.into(),
            url: url.map(|s| s.into()),
            note: note.map(|s| s.into()),
            created_at,
            updated_at,
        }
    }
}

impl<'r> FromRow<'r, PgRow> for BookMaker {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            name: row.get("name"),
            url: Some(row.get("url")),
            note: Some(row.get("note")),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}
