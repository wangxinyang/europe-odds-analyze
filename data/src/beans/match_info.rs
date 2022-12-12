use chrono::NaiveDateTime;

use crate::Odds;

#[derive(Debug)]
pub struct MatchInfo {
    pub id: i32,
    pub league_id: i32,
    pub home_team_id: i32,
    pub away_team_id: i32,
    pub home_team: String,
    pub away_team: String,
    pub game_time: NaiveDateTime,
    pub note: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub odds: Vec<Odds>,
}
