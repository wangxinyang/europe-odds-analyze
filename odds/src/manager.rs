use async_trait::async_trait;
use datas::{BookMaker, League, Matches, Odds, OddsError, Team};
use sqlx::{PgPool, Row};

use crate::{BookMakerId, EuropeOdds, LeagueId, MatchId, OddsManager, TeamId};

impl OddsManager {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl EuropeOdds for OddsManager {
    /// add bookmaker data to persistence
    async fn create_bookermaker(&self, mut bookmaker: BookMaker) -> Result<BookMaker, OddsError> {
        let id = sqlx::query(
            "INSERT INTO euro.bookmakers (name, url, note) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(&bookmaker.name)
        .bind(&bookmaker.url)
        .bind(&bookmaker.note)
        .fetch_one(&self.conn)
        .await?
        .get(0);
        bookmaker.id = id;
        Ok(bookmaker)
    }

    /// update bookmaker data to persistence
    async fn update_bookermaker(&self, bookmaker: BookMaker) -> Result<BookMaker, OddsError> {
        let bm = sqlx::query_as(
            "UPDATE euro.bookmakers SET name = $1, url = $2, note = $3 WHERE id = $4 RETURNING *",
        )
        .bind(&bookmaker.name)
        .bind(&bookmaker.url)
        .bind(&bookmaker.note)
        .bind(bookmaker.id)
        .fetch_one(&self.conn)
        .await?;
        Ok(bm)
    }

    /// delete bookmaker data from persistence
    async fn delete_bookermaker(&self, _id: BookMakerId) -> Result<(), OddsError> {
        todo!()
    }

    /// add league data to persistence
    async fn create_league(&self, _league: League) -> Result<League, OddsError> {
        todo!()
    }

    /// update league data to persistence
    async fn update_league(&self, _league: League) -> Result<League, OddsError> {
        todo!()
    }

    /// delete league data from persistence
    async fn delete_league(&self, _id: LeagueId) -> Result<(), OddsError> {
        todo!()
    }

    /// add team data to persistence
    async fn create_team(&self, _team: Team) -> Result<Team, OddsError> {
        todo!()
    }

    /// update team data to persistence
    async fn update_team(&self, _team: Team) -> Result<Team, OddsError> {
        todo!()
    }

    /// delete team data from persistence
    async fn delete_team(&self, _id: TeamId) -> Result<(), OddsError> {
        todo!()
    }

    /// add match data to persistence
    async fn create_match(&self, _matches: Matches) -> Result<Matches, OddsError> {
        todo!()
    }

    /// update match data to persistence
    async fn update_match(&self, _matches: Matches) -> Result<Matches, OddsError> {
        todo!()
    }

    /// delete match data from persistence
    async fn delete_match(&self, _id: MatchId) -> Result<(), OddsError> {
        todo!()
    }

    /// add odds data to persistence
    async fn create_odds(
        &self,
        _mid: MatchId,
        _bid: BookMakerId,
        _odds: Odds,
    ) -> Result<Matches, OddsError> {
        todo!()
    }

    /// update odds data to persistence
    async fn update_odds(
        &self,
        _mid: MatchId,
        _bid: BookMakerId,
        _odds: Odds,
    ) -> Result<Matches, OddsError> {
        todo!()
    }

    /// delete odds data from persistence
    async fn delete_odds(&self, _mid: MatchId, _bid: BookMakerId) -> Result<(), OddsError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use sqlx::types::chrono::NaiveDateTime;

    use crate::TestConfig;

    use super::*;

    #[tokio::test]
    async fn add_bookmaker_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        let bm = odds_manager
            .create_bookermaker(BookMaker::new_pending(
                "威廉希尔",
                Some("https://sports.williamhill.com/betting/en-gb"),
                Some("第一参考的博彩网站"),
                NaiveDateTime::default(),
                NaiveDateTime::default(),
            ))
            .await
            .unwrap();
        assert!(bm.id != 0);
    }

    #[tokio::test]
    async fn update_bookmaker_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add bookmaker
        let mut bm = odds_manager
            .create_bookermaker(BookMaker::new_pending(
                "威廉希尔",
                Some("https://sports.williamhill.com/betting/en-gb"),
                Some("第一参考的博彩网站"),
                NaiveDateTime::default(),
                NaiveDateTime::default(),
            ))
            .await
            .unwrap();
        // update bookmaker
        bm.name = "威廉希尔1".into();
        let bm1 = odds_manager.update_bookermaker(bm);
        assert_eq!(bm1.await.unwrap().name, "威廉希尔1");
    }
}
