use chrono::NaiveDateTime;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Builder, FromRow, Serialize, Deserialize)]
pub struct Matches {
    #[builder(default)]
    pub id: i32,
    #[builder(default)]
    pub league_id: i32,
    #[builder(default)]
    pub home_team_id: i32,
    #[builder(default)]
    pub away_team_id: i32,
    #[builder(default, setter(into, strip_option))]
    pub home_team: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub away_team: Option<String>,
    #[builder(default, setter(strip_option))]
    pub game_time: Option<NaiveDateTime>,
    #[builder(default, setter(into, strip_option))]
    pub game_year: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub game_round: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub game_result: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub note: Option<String>,
    #[builder(default)]
    pub created_at: NaiveDateTime,
    #[builder(default)]
    pub updated_at: NaiveDateTime,
}
