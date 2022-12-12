use async_trait::async_trait;
use data::{BookMaker, League, MatchInfo, Matches, Odds, OddsError, Team};
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
        println!("step into create_bookermaker");
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
    async fn delete_bookermaker(&self, id: BookMakerId) -> Result<i32, OddsError> {
        let count = sqlx::query("DELETE FROM euro.bookmakers WHERE id = $1 RETURNING *")
            .bind(id)
            .execute(&self.conn)
            .await?;
        Ok(count.rows_affected() as i32)
    }

    /// add league data to persistence
    async fn create_league(&self, mut league: League) -> Result<League, OddsError> {
        let id = sqlx::query("INSERT INTO euro.leagues (name, note) VALUES ($1, $2) RETURNING id")
            .bind(&league.name)
            .bind(&league.note)
            .fetch_one(&self.conn)
            .await?
            .get(0);

        league.id = id;
        Ok(league)
    }

    /// update league data to persistence
    async fn update_league(&self, league: League) -> Result<League, OddsError> {
        let lg = sqlx::query_as(
            "UPDATE euro.leagues SET name = $1, note = $2 WHERE id = $3 RETURNING *",
        )
        .bind(&league.name)
        .bind(&league.note)
        .bind(league.id)
        .fetch_one(&self.conn)
        .await?;
        Ok(lg)
    }

    /// delete league data from persistence
    async fn delete_league(&self, id: LeagueId) -> Result<i32, OddsError> {
        let count = sqlx::query("DELETE FROM euro.leagues WHERE id = $1 RETURNING *")
            .bind(id)
            .execute(&self.conn)
            .await?;
        Ok(count.rows_affected() as i32)
    }

    /// add team data to persistence
    async fn create_team(&self, mut team: Team) -> Result<Team, OddsError> {
        let id = sqlx::query(
            "INSERT INTO euro.teams (name, league_id, note) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(&team.name)
        .bind(team.league_id)
        .bind(&team.note)
        .fetch_one(&self.conn)
        .await?
        .get(0);

        team.id = id;
        Ok(team)
    }

    /// update team data to persistence
    async fn update_team(&self, team: Team) -> Result<Team, OddsError> {
        let tm = sqlx::query_as(
            "UPDATE euro.teams SET name = $1, league_id = $2, note = $3 WHERE id = $4 RETURNING *",
        )
        .bind(&team.name)
        .bind(team.league_id)
        .bind(&team.note)
        .bind(team.id)
        .fetch_one(&self.conn)
        .await?;

        Ok(tm)
    }

    /// delete team data from persistence
    async fn delete_team(&self, id: TeamId) -> Result<i32, OddsError> {
        let count = sqlx::query("DELETE FROM euro.teams WHERE id = $1 RETURNING *")
            .bind(id)
            .execute(&self.conn)
            .await?;

        Ok(count.rows_affected() as i32)
    }

    /// add match data to persistence
    async fn create_match_info(
        &self,
        matches: Matches,
        _odds: Vec<Odds>,
    ) -> Result<MatchInfo, OddsError> {
        // insert matches table
        let _id: i32 = sqlx::query(
            "INSERT INTO euro.matches (league_id, home_team_id, home_team, away_team_id, away_team, game_time, note)
            VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        )
        .bind(matches.league_id)
        .bind(matches.home_team_id)
        .bind(&matches.home_team)
        .bind(matches.away_team_id)
        .bind(&matches.away_team)
        .bind(matches.game_time)
        .bind(&matches.note)
        .fetch_one(&self.conn)
        .await?
        .get(0);

        // insert into odds table

        todo!()
    }

    /// update match data to persistence
    async fn update_match_info(
        &self,
        _matches: Matches,
        _odds: Vec<Odds>,
    ) -> Result<MatchInfo, OddsError> {
        todo!()
    }

    /// delete match data from persistence
    async fn delete_match_info(&self, _id: MatchId) -> Result<i32, OddsError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use data::BookMakerBuilder;

    use crate::test_util::TestConfig;

    use super::*;

    #[tokio::test]
    async fn add_bookmaker_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        let bm = odds_manager
            .create_bookermaker(
                BookMakerBuilder::default()
                    .name("威廉希尔")
                    .url("https://sports.williamhill.com/betting/en-gb")
                    .note("第一参考的博彩网站")
                    .build()
                    .unwrap(),
            )
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
            .create_bookermaker(
                BookMakerBuilder::default()
                    .name("威廉希尔")
                    .url("https://sports.williamhill.com/betting/en-gb")
                    .note("第一参考的博彩网站")
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        // update bookmaker
        bm.name = "威廉希尔1".into();
        let bm1 = odds_manager.update_bookermaker(bm);
        assert_eq!(bm1.await.unwrap().name, "威廉希尔1");
    }

    #[tokio::test]
    async fn delete_bookmaker_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add bookmaker
        let bm = odds_manager
            .create_bookermaker(
                BookMakerBuilder::default()
                    .name("威廉希尔")
                    .url("https://sports.williamhill.com/betting/en-gb")
                    .note("第一参考的博彩网站")
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        // delete bookmaker
        let count = odds_manager.delete_bookermaker(bm.id).await.unwrap();
        assert_eq!(count, 1);
    }
}
