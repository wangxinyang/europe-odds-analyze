use chrono::NaiveDateTime;
use derive_builder::Builder;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Builder, FromRow, Serialize)]
pub struct League {
    #[builder(default)]
    pub id: i32,
    #[builder(setter(into))]
    pub name: String,
    #[builder(default, setter(into, strip_option))]
    pub note: Option<String>,
    #[builder(default)]
    pub created_at: NaiveDateTime,
    #[builder(default)]
    pub updated_at: NaiveDateTime,
}
