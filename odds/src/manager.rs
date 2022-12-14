use async_trait::async_trait;
use data::{BookMaker, DbConfig, League, MatchInfo, Matches, Odds, OddsError, Team};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::{BookMakerId, EuropeOdds, LeagueId, MatchId, OddsManager, TeamId};

impl OddsManager {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }

    pub async fn from_config(config: &DbConfig) -> Result<Self, OddsError> {
        let url = config.url();
        let conn = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&url)
            .await?;
        Ok(Self::new(conn))
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
        mut matches: Matches,
        odds: Vec<Odds>,
    ) -> Result<MatchInfo, OddsError> {
        // insert matches table
        let id: i32 = sqlx::query(
            "INSERT INTO euro.matches (league_id, home_team_id, home_team, away_team_id,
                away_team, game_time, game_result, note)  VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
        )
        .bind(matches.league_id)
        .bind(matches.home_team_id)
        .bind(&matches.home_team)
        .bind(matches.away_team_id)
        .bind(&matches.away_team)
        .bind(matches.game_time)
        .bind(&matches.game_result)
        .bind(&matches.note)
        .fetch_one(&self.conn)
        .await?
        .get(0);
        // set the generated id
        matches.id = id;

        // insert into odds table
        let mut result_odds: Vec<Odds> = Vec::new();
        for mut odd in odds.into_iter() {
            let id = sqlx::query(
                "INSERT INTO euro.odds (match_id, bookmaker_id, home_win_start, draw_start, away_win_start,
                    home_win_end, draw_end, away_win_end, note) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id",
            )
            .bind(id)
            .bind(odd.bookmaker_id)
            .bind(&odd.home_win_start)
            .bind(&odd.draw_start)
            .bind(&odd.away_win_start)
            .bind(&odd.home_win_end)
            .bind(&odd.draw_end)
            .bind(&odd.away_win_end)
            .bind(&odd.note)
            .fetch_one(&self.conn)
            .await?
            .get(0);

            odd.id = id;
            result_odds.push(odd);
        }

        Ok(MatchInfo::new(matches, result_odds))
    }

    /// update match data to persistence
    async fn update_match_info(&self, match_info: MatchInfo) -> Result<MatchInfo, OddsError> {
        let matches = match_info.matches;
        let odds = match_info.odds;
        // update matches table
        let updated_matches = sqlx::query_as::<_, Matches>(
            "UPDATE euro.matches SET league_id = $1, home_team_id = $2, home_team = $3, away_team_id = $4,
                away_team = $5, game_time = $6, game_result = $7, note = $8 WHERE id = $9 RETURNING *",
        )
        .bind(matches.league_id)
        .bind(matches.home_team_id)
        .bind(&matches.home_team)
        .bind(matches.away_team_id)
        .bind(&matches.away_team)
        .bind(matches.game_time)
        .bind(&matches.game_result)
        .bind(&matches.note)
        .bind(matches.id)
        .fetch_one(&self.conn)
        .await?;

        // update odds table
        let mut result_odds: Vec<Odds> = Vec::new();
        for odd in odds.into_iter() {
            let updated_odd = sqlx::query_as::<_, Odds>(
                "UPDATE euro.odds SET bookmaker_id = $1, home_win_start = $2, draw_start = $3,
                    away_win_start = $4, home_win_end = $5, draw_end = $6, away_win_end = $7,
                    note = $8 WHERE match_id = $9 RETURNING *",
            )
            .bind(odd.bookmaker_id)
            .bind(odd.home_win_start)
            .bind(odd.draw_start)
            .bind(odd.away_win_start)
            .bind(odd.home_win_end)
            .bind(odd.draw_end)
            .bind(odd.away_win_end)
            .bind(&odd.note)
            .bind(updated_matches.id)
            .fetch_one(&self.conn)
            .await?;

            result_odds.push(updated_odd);
        }

        Ok(MatchInfo::new(updated_matches, result_odds))
    }

    /// delete match data from persistence
    async fn delete_match_info(&self, id: MatchId) -> Result<i32, OddsError> {
        let count = sqlx::query("DELETE FROM euro.matches WHERE id = $1")
            .bind(id)
            .execute(&self.conn)
            .await?;

        Ok(count.rows_affected() as i32)
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDateTime;
    use data::{BookMakerBuilder, LeagueBuilder, MatchesBuilder, OddsBuilder, TeamBuilder};

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

    #[tokio::test]
    async fn create_league_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let bm = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        assert!(bm.id != 0);
    }

    #[tokio::test]
    async fn update_league_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let mut bm = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // update league info
        bm.name = "英超1".into();
        let bm1 = odds_manager.update_league(bm).await.unwrap();
        assert_eq!(bm1.name, "英超1");
    }

    #[tokio::test]
    async fn delete_league_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let bm = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // delete league info
        let count = odds_manager.delete_league(bm.id).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn create_team_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add team
        let team = odds_manager
            .create_team(
                TeamBuilder::default()
                    .name("曼联")
                    .league_id(1)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(team.id != 0);
        assert_eq!(1, team.league_id);
        assert_eq!("曼联", team.name);
    }

    #[tokio::test]
    async fn update_team_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add team
        let mut team = odds_manager
            .create_team(
                TeamBuilder::default()
                    .name("曼联")
                    .league_id(1)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        // update team info
        team.name = "利物浦".into();
        let team1 = odds_manager.update_team(team).await.unwrap();
        assert_eq!(team1.name, "利物浦");
    }

    #[tokio::test]
    async fn delete_team_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add team
        let team = odds_manager
            .create_team(
                TeamBuilder::default()
                    .name("曼联")
                    .league_id(1)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        // delete team info
        let count = odds_manager.delete_team(team.id).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn create_matches_info_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        let matches = MatchesBuilder::default()
            .league_id(1)
            .home_team_id(1)
            .home_team("曼联")
            .away_team_id(2)
            .away_team("利物浦")
            .game_time(NaiveDateTime::default())
            .game_result("2:1")
            .build()
            .unwrap();
        let odd_1 = OddsBuilder::default()
            .bookmaker_id(1)
            .home_win_start_setter("1.2")
            .draw_start_setter("2.3")
            .away_win_start_setter("3.4")
            .home_win_end_setter("1.5")
            .draw_end_setter("2.6")
            .away_win_end_setter("3.7")
            .build()
            .unwrap();
        let odd_2 = OddsBuilder::default()
            .bookmaker_id(2)
            .home_win_start_setter("1.4")
            .draw_start_setter("2.25")
            .away_win_start_setter("3.45")
            .home_win_end_setter("1.62")
            .draw_end_setter("2.65")
            .away_win_end_setter("3.72")
            .build()
            .unwrap();
        let odds = vec![odd_1, odd_2];

        // add match info
        let match_info = odds_manager.create_match_info(matches, odds).await.unwrap();
        assert!(match_info.matches.id != 0);
        assert_eq!(match_info.odds.len(), 2);
    }

    #[tokio::test]
    async fn update_matches_info_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add match info
        let matches = MatchesBuilder::default()
            .league_id(1)
            .home_team_id(1)
            .home_team("曼联")
            .away_team_id(2)
            .away_team("利物浦")
            .game_time(NaiveDateTime::default())
            .game_result("2:1")
            .build()
            .unwrap();
        let odd_1 = OddsBuilder::default()
            .bookmaker_id(1)
            .home_win_start_setter("1.2")
            .draw_start_setter("2.3")
            .away_win_start_setter("3.4")
            .home_win_end_setter("1.5")
            .draw_end_setter("2.6")
            .away_win_end_setter("3.7")
            .build()
            .unwrap();
        let odd_2 = OddsBuilder::default()
            .bookmaker_id(2)
            .home_win_start_setter("1.4")
            .draw_start_setter("2.25")
            .away_win_start_setter("3.45")
            .home_win_end_setter("1.62")
            .draw_end_setter("2.65")
            .away_win_end_setter("3.72")
            .build()
            .unwrap();
        let odds = vec![odd_1, odd_2];
        let mut match_info = odds_manager.create_match_info(matches, odds).await.unwrap();

        // update match info
        match_info.matches.game_result = Some("1:1".into());
        match_info.odds[0].home_win_start = "3.01".parse().unwrap();
        match_info.odds[1].home_win_start = "4.05".parse().unwrap();
        let match_info = odds_manager.update_match_info(match_info).await.unwrap();

        assert_eq!(match_info.matches.game_result.unwrap(), "1:1");
        assert_eq!(match_info.odds[0].home_win_start, "3.01".parse().unwrap());
        assert_eq!(match_info.odds[1].home_win_start, "4.05".parse().unwrap());
    }

    #[tokio::test]
    async fn delete_matches_info_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add match info
        let matches = MatchesBuilder::default()
            .league_id(1)
            .home_team_id(1)
            .home_team("曼联")
            .away_team_id(2)
            .away_team("利物浦")
            .game_time(NaiveDateTime::default())
            .game_result("2:1")
            .build()
            .unwrap();
        let odd_1 = OddsBuilder::default()
            .bookmaker_id(1)
            .home_win_start_setter("1.2")
            .draw_start_setter("2.3")
            .away_win_start_setter("3.4")
            .home_win_end_setter("1.5")
            .draw_end_setter("2.6")
            .away_win_end_setter("3.7")
            .build()
            .unwrap();
        let odd_2 = OddsBuilder::default()
            .bookmaker_id(2)
            .home_win_start_setter("1.4")
            .draw_start_setter("2.25")
            .away_win_start_setter("3.45")
            .home_win_end_setter("1.62")
            .draw_end_setter("2.65")
            .away_win_end_setter("3.72")
            .build()
            .unwrap();
        let odds = vec![odd_1, odd_2];
        let match_info = odds_manager.create_match_info(matches, odds).await.unwrap();

        // delete match info
        let delete_count = odds_manager
            .delete_match_info(match_info.matches.id)
            .await
            .unwrap();

        assert_eq!(1, delete_count);
    }
}
