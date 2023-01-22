use async_trait::async_trait;
use data::{BookMaker, DbConfig, League, MatchInfoQuery, Matches, Odds, OddsError, Team};
use sqlx::{postgres::PgPoolOptions, PgPool, Row};

use crate::{BookMakerId, EuropeOdds, LeagueId, MatchId, OddId, OddsManager, TeamId};

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
    async fn list_bookermaker(&self) -> Result<Vec<BookMaker>, OddsError> {
        let book_makers = sqlx::query_as("SELECT * FROM euro.bookmakers ORDER BY created_at ASC")
            .fetch_all(&self.conn)
            .await?;

        Ok(book_makers)
    }

    /// add bookmaker data to persistence
    async fn create_bookermaker(
        &self,
        mut bookmaker: BookMaker,
    ) -> Result<Vec<BookMaker>, OddsError> {
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

        let book_makers = self.list_bookermaker().await?;
        Ok(book_makers)
    }

    /// update bookmaker data to persistence
    async fn update_bookermaker(&self, bookmaker: BookMaker) -> Result<Vec<BookMaker>, OddsError> {
        sqlx::query(
            "UPDATE euro.bookmakers SET name = $1, url = $2, note = $3 WHERE id = $4 RETURNING *",
        )
        .bind(&bookmaker.name)
        .bind(&bookmaker.url)
        .bind(&bookmaker.note)
        .bind(bookmaker.id)
        .fetch_one(&self.conn)
        .await?;

        let book_makers = self.list_bookermaker().await?;
        Ok(book_makers)
    }

    /// delete bookmaker data from persistence
    async fn delete_bookermaker(&self, id: BookMakerId) -> Result<Vec<BookMaker>, OddsError> {
        sqlx::query("DELETE FROM euro.bookmakers WHERE id = $1 RETURNING *")
            .bind(id)
            .execute(&self.conn)
            .await?;

        let book_makers = self.list_bookermaker().await?;
        Ok(book_makers)
    }

    /// query bookmaker data by id
    async fn query_bookermaker_with_id(&self, id: BookMakerId) -> Result<BookMaker, OddsError> {
        let book_makers = sqlx::query_as("SELECT * FROM euro.bookmakers where id = $1")
            .bind(id)
            .fetch_one(&self.conn)
            .await?;

        Ok(book_makers)
    }

    /// get all league data
    async fn list_leagues(&self) -> Result<Vec<League>, OddsError> {
        let leagues = sqlx::query_as("SELECT * FROM euro.leagues ORDER BY created_at ASC")
            .fetch_all(&self.conn)
            .await?;

        Ok(leagues)
    }

    /// query league data by id
    async fn query_league_with_id(&self, id: LeagueId) -> Result<League, OddsError> {
        let league = sqlx::query_as("SELECT * FROM euro.leagues where id = $1")
            .bind(id)
            .fetch_one(&self.conn)
            .await?;

        Ok(league)
    }

    /// add league data to persistence
    async fn create_league(&self, mut league: League) -> Result<Vec<League>, OddsError> {
        let id = sqlx::query("INSERT INTO euro.leagues (name, note) VALUES ($1, $2) RETURNING id")
            .bind(&league.name)
            .bind(&league.note)
            .fetch_one(&self.conn)
            .await?
            .get(0);

        league.id = id;
        let leagues = self.list_leagues().await?;
        Ok(leagues)
    }

    /// update league data to persistence
    async fn update_league(&self, league: League) -> Result<Vec<League>, OddsError> {
        sqlx::query("UPDATE euro.leagues SET name = $1, note = $2 WHERE id = $3 RETURNING *")
            .bind(&league.name)
            .bind(&league.note)
            .bind(league.id)
            .fetch_one(&self.conn)
            .await?;

        let leagues = self.list_leagues().await?;
        Ok(leagues)
    }

    /// delete league data from persistence
    async fn delete_league(&self, id: LeagueId) -> Result<Vec<League>, OddsError> {
        sqlx::query("DELETE FROM euro.leagues WHERE id = $1 RETURNING *")
            .bind(id)
            .execute(&self.conn)
            .await?;

        let leagues = self.list_leagues().await?;
        Ok(leagues)
    }

    /// get all team data
    async fn list_teams(&self) -> Result<Vec<Team>, OddsError> {
        let teams = sqlx::query_as(
            "SELECT teams.*, leagues.name league_name FROM euro.teams teams,
            euro.leagues leagues where teams.league_id = leagues.id ORDER BY teams.created_at ASC",
        )
        .fetch_all(&self.conn)
        .await?;

        Ok(teams)
    }

    /// query team data by id
    async fn query_team_with_id(&self, id: TeamId) -> Result<Team, OddsError> {
        let team = sqlx::query_as(
            "SELECT teams.*, leagues.name league_name FROM euro.teams teams
        , euro.leagues leagues where teams.league_id = leagues.id AND teams.id = $1",
        )
        .bind(id)
        .fetch_one(&self.conn)
        .await?;
        Ok(team)
    }

    /// query team data by league id
    async fn query_teams_with_condition(&self, id: LeagueId) -> Result<Vec<Team>, OddsError> {
        let teams = sqlx::query_as(
            "SELECT teams.*, leagues.name league_name FROM euro.teams teams,
            euro.leagues leagues where teams.league_id = leagues.id and leagues.id = $1 ORDER BY teams.created_at DESC",
        )
        .bind(id)
        .fetch_all(&self.conn)
        .await?;

        Ok(teams)
    }

    /// add team data to persistence
    async fn create_team(&self, mut team: Team) -> Result<Vec<Team>, OddsError> {
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

        let teams = self.list_teams().await?;
        Ok(teams)
    }

    /// update team data to persistence
    async fn update_team(&self, team: Team) -> Result<Vec<Team>, OddsError> {
        sqlx::query(
            "UPDATE euro.teams SET name = $1, league_id = $2, note = $3 WHERE id = $4 RETURNING *",
        )
        .bind(&team.name)
        .bind(team.league_id)
        .bind(&team.note)
        .bind(team.id)
        .fetch_one(&self.conn)
        .await?;

        let teams = self.list_teams().await?;
        Ok(teams)
    }

    /// delete team data from persistence
    async fn delete_team(&self, id: TeamId) -> Result<Vec<Team>, OddsError> {
        sqlx::query("DELETE FROM euro.teams WHERE id = $1 RETURNING *")
            .bind(id)
            .execute(&self.conn)
            .await?;

        let teams = self.list_teams().await?;
        Ok(teams)
    }

    /// add match data to persistence
    async fn create_match_info(&self, mut matches: Matches) -> Result<Matches, OddsError> {
        // insert matches table
        let id: i32 = sqlx::query(
            "INSERT INTO euro.matches (league_id, league_name, home_team_id, home_team, away_team_id,
                away_team, game_time, game_year, game_round, game_result, history_note, note, predict_game_result)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) RETURNING id",
        )
        .bind(matches.league_id)
        .bind(&matches.league_name)
        .bind(matches.home_team_id)
        .bind(&matches.home_team)
        .bind(matches.away_team_id)
        .bind(&matches.away_team)
        .bind(matches.game_time)
        .bind(&matches.game_year)
        .bind(&matches.game_round)
        .bind(&matches.game_result)
        .bind(&matches.history_note)
        .bind(&matches.note)
        .bind(&matches.predict_game_result)
        .fetch_one(&self.conn)
        .await?
        .get(0);
        // set the generated id
        matches.id = id;

        Ok(matches)
    }

    /// update match data to persistence
    async fn update_match_info(&self, matches: Matches) -> Result<Matches, OddsError> {
        // update matches table
        let matches = sqlx::query_as::<_, Matches>(
            "UPDATE euro.matches SET league_id = $1,
        home_team_id = $2, home_team = $3, away_team_id = $4, away_team = $5, game_time = $6,
        game_result = $7, note = $8, game_year = $9, game_round = $10,
        league_name = $11, history_note = $12, predict_game_result = $13 WHERE id = $14 RETURNING *",
        )
        .bind(matches.league_id)
        .bind(matches.home_team_id)
        .bind(&matches.home_team)
        .bind(matches.away_team_id)
        .bind(&matches.away_team)
        .bind(matches.game_time)
        .bind(&matches.game_result)
        .bind(&matches.note)
        .bind(&matches.game_year)
        .bind(&matches.game_round)
        .bind(&matches.league_name)
        .bind(&matches.history_note)
        .bind(&matches.predict_game_result)
        .bind(matches.id)
        .fetch_one(&self.conn)
        .await?;

        Ok(matches)
    }

    /// delete match data from persistence
    async fn delete_match_info(&self, id: MatchId) -> Result<i32, OddsError> {
        let count = sqlx::query("DELETE FROM euro.matches WHERE id = $1")
            .bind(id)
            .execute(&self.conn)
            .await?;

        Ok(count.rows_affected() as i32)
    }

    /// query match data by conditions
    async fn query_match_info(&self, query: MatchInfoQuery) -> Result<Vec<Matches>, OddsError> {
        let match_infos =
            sqlx::query_as("select * from euro.query($1, $2, $3, $4, $5, $6, $7, $8)")
                .bind(query.book_maker_id)
                .bind(query.league_id)
                .bind(query.team_id)
                .bind(query.game_year)
                .bind(query.game_round)
                .bind(query.is_desc)
                .bind(query.cursor)
                .bind(query.page_size)
                .fetch_all(&self.conn)
                .await?;
        Ok(match_infos)
    }

    /// query odds data by match id
    async fn query_odds_info_by_id(&self, id: i32) -> Result<Vec<Odds>, OddsError> {
        let odds_infos = sqlx::query_as("select * from euro.odds where match_id = $1")
            .bind(id)
            .fetch_all(&self.conn)
            .await?;
        Ok(odds_infos)
    }

    /// add match data to persistence
    async fn create_odd_info(&self, id: MatchId, mut odd: Odds) -> Result<Odds, OddsError> {
        // insert into odds table
        let id = sqlx::query("INSERT INTO euro.odds (match_id, bookmaker_id, bookmaker_name,
                home_win_start, draw_start, away_win_start,home_win_end, draw_end, away_win_end, note)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id")
                    .bind(id)
                    .bind(odd.bookmaker_id)
                    .bind(&odd.bookmaker_name)
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
        Ok(odd)
    }

    /// update match data to persistence
    async fn update_odd_info(&self, odd: Odds) -> Result<Odds, OddsError> {
        let odd_info = sqlx::query_as::<_, Odds>(
            "UPDATE euro.odds SET bookmaker_id = $1, home_win_start = $2, draw_start = $3,
             away_win_start = $4, home_win_end = $5, draw_end = $6, away_win_end = $7, note = $8,
             bookmaker_name= $9 WHERE match_id = $10 and id = $11 RETURNING *",
        )
        .bind(odd.bookmaker_id)
        .bind(odd.home_win_start)
        .bind(odd.draw_start)
        .bind(odd.away_win_start)
        .bind(odd.home_win_end)
        .bind(odd.draw_end)
        .bind(odd.away_win_end)
        .bind(&odd.note)
        .bind(&odd.bookmaker_name)
        .bind(odd.match_id)
        .bind(odd.id)
        .fetch_one(&self.conn)
        .await?;

        Ok(odd_info)
    }

    /// delete match data from persistence
    async fn delete_odds_info(&self, id: OddId) -> Result<i32, OddsError> {
        let count = sqlx::query("DELETE FROM euro.odds WHERE id = $1")
            .bind(id)
            .execute(&self.conn)
            .await?;

        Ok(count.rows_affected() as i32)
    }
}

#[cfg(test)]
mod tests {

    use chrono::NaiveDateTime;
    use data::{BookMakerBuilder, LeagueBuilder, MatchesBuilder, TeamBuilder};

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
        assert_eq!(bm.len(), 1);
    }

    #[tokio::test]
    async fn update_bookmaker_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add bookmaker
        let mut bms = odds_manager
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
        let mut bm = bms.pop().unwrap();
        bm.name = "威廉希尔1".into();
        let mut bm1 = odds_manager.update_bookermaker(bm).await.unwrap();
        assert_eq!(bm1.pop().unwrap().name, "威廉希尔1");
    }

    #[tokio::test]
    async fn delete_bookmaker_should_be_work() {
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
        // delete bookmaker
        let bms = odds_manager
            .delete_bookermaker(bm.pop().unwrap().id)
            .await
            .unwrap();
        assert_eq!(bms.len(), 0);
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
        assert_eq!(bm.len(), 1);
    }

    #[tokio::test]
    async fn update_league_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let mut bms = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // update league info
        let mut bm = bms.pop().unwrap();
        bm.name = "英超1".into();
        let bm1 = odds_manager.update_league(bm).await.unwrap();
        assert_eq!(bm1.get(0).unwrap().name, "英超1");
    }

    #[tokio::test]
    async fn delete_league_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let mut bms = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // delete league info
        let bm = bms.pop().unwrap();
        let bm1 = odds_manager.delete_league(bm.id).await.unwrap();
        assert_eq!(bm1.len(), 0);
    }

    #[tokio::test]
    async fn create_team_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let mut leagues = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // add team
        let mut teams = odds_manager
            .create_team(
                TeamBuilder::default()
                    .name("曼联")
                    .league_id(leagues.pop().unwrap().id)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(teams.len(), 1);
        let team = teams.pop().unwrap();
        assert_eq!(1, team.league_id);
        assert_eq!("曼联", team.name);
    }

    #[tokio::test]
    async fn update_team_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let mut leagues = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // add team
        let mut teams = odds_manager
            .create_team(
                TeamBuilder::default()
                    .name("曼联")
                    .league_id(leagues.pop().unwrap().id)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        // update team info
        let mut team = teams.pop().unwrap();
        team.name = "利物浦".into();
        let team1 = odds_manager.update_team(team).await.unwrap();
        assert_eq!(team1.get(0).unwrap().name, "利物浦");
    }

    #[tokio::test]
    async fn delete_team_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add league
        let mut leagues = odds_manager
            .create_league(LeagueBuilder::default().name("英超").build().unwrap())
            .await
            .unwrap();
        // add team
        let mut teams = odds_manager
            .create_team(
                TeamBuilder::default()
                    .name("曼联")
                    .league_id(leagues.pop().unwrap().id)
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();
        // delete team info
        let team = teams.pop().unwrap();
        let team1 = odds_manager.delete_team(team.id).await.unwrap();
        assert_eq!(team1.len(), 0);
    }

    #[tokio::test]
    async fn create_matches_info_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        let matches = MatchesBuilder::default()
            .league_id(1)
            .league_name("英超")
            .home_team_id(1)
            .home_team("曼联")
            .away_team_id(2)
            .away_team("利物浦")
            .game_time(NaiveDateTime::default())
            .game_result("2:1")
            .game_round("1")
            .game_year("2022")
            .build()
            .unwrap();
        // let odd_1 = OddsBuilder::default()
        //     .bookmaker_id(1)
        //     .home_win_start_setter("1.2")
        //     .draw_start_setter("2.3")
        //     .away_win_start_setter("3.4")
        //     .home_win_end_setter("1.5")
        //     .draw_end_setter("2.6")
        //     .away_win_end_setter("3.7")
        //     .build()
        //     .unwrap();
        // let odd_2 = OddsBuilder::default()
        //     .bookmaker_id(2)
        //     .home_win_start_setter("1.4")
        //     .draw_start_setter("2.25")
        //     .away_win_start_setter("3.45")
        //     .home_win_end_setter("1.62")
        //     .draw_end_setter("2.65")
        //     .away_win_end_setter("3.72")
        //     .build()
        //     .unwrap();
        // let odds = vec![odd_1, odd_2];

        // add match info
        let match_info = odds_manager.create_match_info(matches).await.unwrap();
        assert!(match_info.id != 0);
    }

    #[tokio::test]
    async fn update_matches_info_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add match info
        let matches = MatchesBuilder::default()
            .league_id(1)
            .league_name("英超")
            .home_team_id(1)
            .home_team("曼联")
            .away_team_id(2)
            .away_team("利物浦")
            .game_time(NaiveDateTime::default())
            .game_result("2:1")
            .build()
            .unwrap();
        // let odd_1 = OddsBuilder::default()
        //     .bookmaker_id(1)
        //     .home_win_start_setter("1.2")
        //     .draw_start_setter("2.3")
        //     .away_win_start_setter("3.4")
        //     .home_win_end_setter("1.5")
        //     .draw_end_setter("2.6")
        //     .away_win_end_setter("3.7")
        //     .build()
        //     .unwrap();
        // let odd_2 = OddsBuilder::default()
        //     .bookmaker_id(2)
        //     .home_win_start_setter("1.4")
        //     .draw_start_setter("2.25")
        //     .away_win_start_setter("3.45")
        //     .home_win_end_setter("1.62")
        //     .draw_end_setter("2.65")
        //     .away_win_end_setter("3.72")
        //     .build()
        //     .unwrap();
        // let odds = vec![odd_1, odd_2];
        let mut match_info = odds_manager.create_match_info(matches).await.unwrap();
        // update match info
        match_info.game_result = Some("1:1".into());
        let update_match = odds_manager.update_match_info(match_info).await.unwrap();
        assert_eq!(update_match.game_result.unwrap(), "1:1");
        // assert_eq!(match_info.odds[0].home_win_start, "3.01".parse().unwrap());
        // assert_eq!(match_info.odds[1].home_win_start, "4.05".parse().unwrap());
    }

    #[tokio::test]
    async fn delete_matches_info_should_be_work() {
        let config = TestConfig::new().await;
        let odds_manager = OddsManager::new(config.tps.get_pool().await);
        // add match info
        let matches = MatchesBuilder::default()
            .league_id(1)
            .league_name("英超")
            .home_team_id(1)
            .home_team("曼联")
            .away_team_id(2)
            .away_team("利物浦")
            .game_time(NaiveDateTime::default())
            .game_result("2:1")
            .build()
            .unwrap();
        // let odd_1 = OddsBuilder::default()
        //     .bookmaker_id(1)
        //     .home_win_start_setter("1.2")
        //     .draw_start_setter("2.3")
        //     .away_win_start_setter("3.4")
        //     .home_win_end_setter("1.5")
        //     .draw_end_setter("2.6")
        //     .away_win_end_setter("3.7")
        //     .build()
        //     .unwrap();
        // let odd_2 = OddsBuilder::default()
        //     .bookmaker_id(2)
        //     .home_win_start_setter("1.4")
        //     .draw_start_setter("2.25")
        //     .away_win_start_setter("3.45")
        //     .home_win_end_setter("1.62")
        //     .draw_end_setter("2.65")
        //     .away_win_end_setter("3.72")
        //     .build()
        //     .unwrap();
        // let odds = vec![odd_1, odd_2];
        let match_info = odds_manager.create_match_info(matches).await.unwrap();

        // delete match info
        let delete_count = odds_manager.delete_match_info(match_info.id).await.unwrap();

        assert_eq!(1, delete_count);
    }
}
