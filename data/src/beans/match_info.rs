use serde::{Deserialize, Serialize};

use crate::{Matches, Odds};

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchInfoQuery {
    pub book_maker_id: i32,
    pub league_id: i32,
    pub team_id: i32,
    pub game_year: Option<String>,
    pub game_round: Option<String>,
    pub is_desc: bool,
    pub cursor: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchInfo {
    pub matches: Matches,
    pub odds: Vec<Odds>,
}

impl MatchInfo {
    pub fn new(matches: Matches, odds: Vec<Odds>) -> Self {
        Self { matches, odds }
    }
}
