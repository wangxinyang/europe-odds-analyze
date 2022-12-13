mod manager;
mod test_util;

pub use manager::*;
pub use test_util::*;

use async_trait::async_trait;
use data::{BookMaker, League, MatchInfo, Matches, Odds, OddsError, Team};
use sqlx::PgPool;

type BookMakerId = i32;
type LeagueId = i32;
type TeamId = i32;
type MatchId = i32;

#[async_trait]
pub trait EuropeOdds {
    /// add bookmaker data to persistence
    async fn create_bookermaker(&self, bookmaker: BookMaker) -> Result<BookMaker, OddsError>;

    /// update bookmaker data to persistence
    async fn update_bookermaker(&self, bookmaker: BookMaker) -> Result<BookMaker, OddsError>;

    /// delete bookmaker data from persistence
    async fn delete_bookermaker(&self, id: BookMakerId) -> Result<i32, OddsError>;

    /// add league data to persistence
    async fn create_league(&self, league: League) -> Result<League, OddsError>;

    /// update league data to persistence
    async fn update_league(&self, league: League) -> Result<League, OddsError>;

    /// delete league data from persistence
    async fn delete_league(&self, id: LeagueId) -> Result<i32, OddsError>;

    /// add team data to persistence
    async fn create_team(&self, team: Team) -> Result<Team, OddsError>;

    /// update team data to persistence
    async fn update_team(&self, team: Team) -> Result<Team, OddsError>;

    /// delete team data from persistence
    async fn delete_team(&self, id: TeamId) -> Result<i32, OddsError>;

    /// add match data to persistence
    async fn create_match_info(
        &self,
        matches: Matches,
        odds: Vec<Odds>,
    ) -> Result<MatchInfo, OddsError>;

    /// update match data to persistence
    async fn update_match_info(&self, match_info: MatchInfo) -> Result<MatchInfo, OddsError>;

    /// delete match data from persistence
    async fn delete_match_info(&self, id: MatchId) -> Result<i32, OddsError>;
}

pub struct OddsManager {
    pub conn: PgPool,
}
