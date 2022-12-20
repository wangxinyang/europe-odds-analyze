use serde::Serialize;

use crate::{Matches, Odds};

#[derive(Debug, Serialize)]
pub struct MatchInfo {
    pub matches: Matches,
    pub odds: Vec<Odds>,
}

impl MatchInfo {
    pub fn new(matches: Matches, odds: Vec<Odds>) -> Self {
        Self { matches, odds }
    }
}
