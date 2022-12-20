use chrono::NaiveDateTime;
use data::{MatchInfo, MatchesBuilder, Odds, OddsBuilder, OddsError, Team};
use odds::{EuropeOdds, OddsManager};
use serde::Deserialize;
use tauri::State;

#[tauri::command]
pub async fn query_teams_with_league(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<Vec<Team>, OddsError> {
    let manager = &*manager;
    let teams = manager.query_teams_with_league(id).await?;
    Ok(teams)
}

#[derive(Debug, Deserialize)]
pub struct OddsInfo {
    pub bookmaker_id: i32,
    pub home_win_start: String,
    pub draw_start: String,
    pub away_win_start: String,
    pub home_win_end: String,
    pub draw_end: String,
    pub away_win_end: String,
}

#[derive(Debug, Deserialize)]
pub struct MatchOddsInfo {
    pub league_id: i32,
    pub home_team_id: i32,
    pub home_team_name: String,
    pub away_team_id: i32,
    pub away_team_name: String,
    pub game_time: String,
    pub game_year: String,
    pub game_round: String,
    pub game_result: String,
    pub note: String,
}

// Result<Vec<MatchInfo>, OddsError>
#[tauri::command]
pub async fn save_match_odds(
    manager: State<'_, OddsManager>,
    match_info: MatchOddsInfo,
    odds_infos: Vec<OddsInfo>,
) -> Result<MatchInfo, OddsError> {
    let manager = &*manager;
    let m_info = MatchesBuilder::default()
        .league_id(match_info.league_id)
        .home_team_id(match_info.home_team_id)
        .away_team_id(match_info.away_team_id)
        .home_team(match_info.home_team_name)
        .away_team(match_info.away_team_name)
        .game_time(
            NaiveDateTime::parse_from_str(match_info.game_time.as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap(),
        )
        .game_year(match_info.game_year)
        .game_round(match_info.game_round)
        .game_result(match_info.game_result)
        .note(match_info.note)
        .build()
        .unwrap();
    let odds_infos: Vec<Odds> = odds_infos
        .iter()
        .map(|info| {
            OddsBuilder::default()
                .bookmaker_id(info.bookmaker_id)
                .home_win_start_setter(info.home_win_start.as_str())
                .draw_start_setter(info.draw_start.as_str())
                .away_win_start_setter(info.away_win_start.as_str())
                .home_win_end_setter(info.home_win_end.as_str())
                .draw_end_setter(info.draw_end.as_str())
                .away_win_end_setter(info.away_win_end.as_str())
                .build()
                .unwrap()
        })
        .collect();
    let info = manager.create_match_info(m_info, odds_infos).await?;
    Ok(info)
}
