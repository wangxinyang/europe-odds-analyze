use chrono::NaiveDateTime;
use data::{
    MatchInfo, MatchInfoQuery, Matches, MatchesBuilder, Odds, OddsBuilder, OddsError, Team,
};
use odds::{EuropeOdds, OddsManager};
use serde::Deserialize;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct OddsInfo {
    pub bookmaker_id: i32,
    pub home_win_start: Option<String>,
    pub draw_start: Option<String>,
    pub away_win_start: Option<String>,
    pub home_win_end: Option<String>,
    pub draw_end: Option<String>,
    pub away_win_end: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatchOddsInfo {
    pub league_id: i32,
    pub home_team_id: i32,
    pub home_team_name: String,
    pub away_team_id: i32,
    pub away_team_name: String,
    pub game_time: String,
    pub game_year: Option<String>,
    pub game_round: Option<String>,
    pub game_result: Option<String>,
    pub note: Option<String>,
}

#[tauri::command]
pub async fn query_teams_with_league(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<Vec<Team>, OddsError> {
    let manager = &*manager;
    let teams = manager.query_teams_with_league(id).await?;
    Ok(teams)
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
        .game_year(match_info.game_year.unwrap_or_default())
        .game_round(match_info.game_round.unwrap_or_default())
        .game_result(match_info.game_result.unwrap_or_default())
        .note(match_info.note.unwrap_or_default())
        .build()
        .unwrap();
    let odds_infos: Vec<Odds> = odds_infos
        .iter()
        .map(|info| {
            OddsBuilder::default()
                .bookmaker_id(info.bookmaker_id)
                .home_win_start_setter(
                    info.home_win_start
                        .clone()
                        .unwrap_or_else(|| "0.00".to_string())
                        .as_str(),
                )
                .draw_start_setter(
                    info.draw_start
                        .clone()
                        .unwrap_or_else(|| "0.00".to_string())
                        .as_str(),
                )
                .away_win_start_setter(
                    info.away_win_start
                        .clone()
                        .unwrap_or_else(|| "0.00".to_string())
                        .as_str(),
                )
                .home_win_end_setter(
                    info.home_win_end
                        .clone()
                        .unwrap_or_else(|| "0.00".to_string())
                        .as_str(),
                )
                .draw_end_setter(
                    info.draw_end
                        .clone()
                        .unwrap_or_else(|| "0.00".to_string())
                        .as_str(),
                )
                .away_win_end_setter(
                    info.away_win_end
                        .clone()
                        .unwrap_or_else(|| "0.00".to_string())
                        .as_str(),
                )
                .build()
                .unwrap()
        })
        .collect();
    let info = manager.create_match_info(m_info, odds_infos).await?;
    Ok(info)
}

#[tauri::command]
pub async fn query_match_info(
    manager: State<'_, OddsManager>,
    query: MatchInfoQuery,
) -> Result<Vec<Matches>, OddsError> {
    println!("query is: {:?}", query);
    let manager = &*manager;
    let match_info = manager.query_match_info(query).await?;
    println!("match_info is: {:?}", match_info);
    Ok(match_info)
}
