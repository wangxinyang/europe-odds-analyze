use chrono::NaiveDateTime;
use derive_builder::Builder;
use sqlx::FromRow;

#[derive(Debug, Builder, FromRow)]
pub struct Matches {
    #[builder(default)]
    pub id: i32,
    #[builder(default)]
    pub league_id: i32,
    #[builder(default)]
    pub home_team_id: i32,
    #[builder(default)]
    pub away_team_id: i32,
    #[builder(setter(into))]
    pub home_team: String,
    #[builder(setter(into))]
    pub away_team: String,
    #[builder(setter(strip_option))]
    pub game_time: NaiveDateTime,
    #[builder(setter(into, strip_option))]
    pub note: Option<String>,
    #[builder(default)]
    pub created_at: NaiveDateTime,
    #[builder(default)]
    pub updated_at: NaiveDateTime,
}
