use chrono::NaiveDateTime;
use data::{
    MatchInfo, MatchInfoQuery, Matches, MatchesBuilder, Odds, OddsBuilder, OddsError, Team,
};
use odds::{EuropeOdds, OddsManager};
use serde::Deserialize;
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct OddsInfo {
    pub id: i32,
    pub match_id: i32,
    pub bookmaker_id: i32,
    pub bookmaker_name: String,
    pub home_win_start: Option<String>,
    pub draw_start: Option<String>,
    pub away_win_start: Option<String>,
    pub home_win_end: Option<String>,
    pub draw_end: Option<String>,
    pub away_win_end: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MatchesInfo {
    pub id: i32,
    pub league_id: i32,
    pub league_name: String,
    pub home_team_id: i32,
    pub home_team_name: String,
    pub away_team_id: i32,
    pub away_team_name: String,
    pub game_time: Option<String>,
    pub game_year: Option<String>,
    pub game_round: Option<String>,
    pub game_result: Option<String>,
    pub history_note: Option<String>,
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
    match_info: MatchesInfo,
    odds_infos: Vec<OddsInfo>,
) -> Result<MatchInfo, OddsError> {
    let manager = &*manager;
    let (m_info, o_infos) = builde_match_odds_info(match_info, odds_infos);
    // insert match info
    let match_info = manager.create_match_info(m_info).await?;
    // insert odds info
    let mut result_odds_info = vec![];
    for info in o_infos {
        let odds_info = manager.create_odd_info(match_info.id, info).await?;
        result_odds_info.push(odds_info)
    }

    Ok(MatchInfo::new(match_info, result_odds_info))
}

#[tauri::command]
pub async fn query_match_info(
    manager: State<'_, OddsManager>,
    query: MatchInfoQuery,
) -> Result<Vec<Matches>, OddsError> {
    let manager = &*manager;
    let match_info = manager.query_match_info(query).await?;
    Ok(match_info)
}

#[tauri::command]
pub async fn delete_match_info(manager: State<'_, OddsManager>, id: i32) -> Result<i32, OddsError> {
    let manager = &*manager;
    let count = manager.delete_match_info(id).await?;
    Ok(count)
}

#[tauri::command]
pub async fn query_odds_by_id(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<Vec<Odds>, OddsError> {
    let manager = &*manager;
    let mut odds = manager.query_odds_info_by_id(id).await?;
    let mut new_result = vec![];
    for odd in odds.iter_mut() {
        odd.home_win_start = odd.home_win_start.with_prec(3);
        odd.home_win_end = odd.home_win_end.with_prec(3);
        odd.draw_start = odd.draw_start.with_prec(3);
        odd.draw_end = odd.draw_end.with_prec(3);
        odd.away_win_start = odd.away_win_start.with_prec(3);
        odd.away_win_end = odd.away_win_end.with_prec(3);
        new_result.push(odd.clone());
    }
    Ok(new_result)
}

// Result<Vec<MatchInfo>, OddsError>
#[tauri::command]
pub async fn update_match_odds(
    manager: State<'_, OddsManager>,
    match_info: MatchesInfo,
    odds_infos: Vec<OddsInfo>,
) -> Result<(), OddsError> {
    let manager = &*manager;
    let (m_info, o_infos) = builde_match_odds_info(match_info, odds_infos);
    // update match info
    let match_info = manager.update_match_info(m_info).await?;
    // query odds info with now database
    let mut odds = manager.query_odds_info_by_id(match_info.id).await?;
    // odds info insert or update or delete with the odds id in the odds info data
    for info in o_infos.into_iter() {
        let id = info.id;
        match odds.clone().into_iter().position(|odd| odd.id == id) {
            Some(index) => {
                // update the info
                manager.update_odd_info(info).await?;
                odds.remove(index);
            }
            None => {
                if id == 0 {
                    // insert into odds table
                    manager.create_odd_info(match_info.id, info).await?;
                }
            }
        }
    }
    for old_item in odds.iter() {
        // delete the old data
        manager.delete_odds_info(old_item.id).await?;
    }
    Ok(())
}

fn builde_match_odds_info(
    match_info: MatchesInfo,
    odds_infos: Vec<OddsInfo>,
) -> (Matches, Vec<Odds>) {
    let game_time = match match_info.game_time {
        Some(time) => NaiveDateTime::parse_from_str(time.as_str(), "%Y-%m-%d %H:%M:%S").unwrap(),
        None => NaiveDateTime::default(),
    };
    let m_info = MatchesBuilder::default()
        .id(match_info.id)
        .league_id(match_info.league_id)
        .league_name(match_info.league_name)
        .home_team_id(match_info.home_team_id)
        .away_team_id(match_info.away_team_id)
        .home_team(match_info.home_team_name)
        .away_team(match_info.away_team_name)
        .game_time(game_time)
        .game_year(match_info.game_year.unwrap_or_default())
        .game_round(match_info.game_round.unwrap_or_default())
        .game_result(match_info.game_result.unwrap_or_default())
        .history_note(match_info.history_note.unwrap_or_default())
        .note(match_info.note.unwrap_or_default())
        .build()
        .unwrap();
    let odds_infos: Vec<Odds> = odds_infos
        .iter()
        .map(|info| {
            OddsBuilder::default()
                .id(info.id)
                .match_id(info.match_id)
                .bookmaker_id(info.bookmaker_id)
                .bookmaker_name(info.bookmaker_name.clone())
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
    (m_info, odds_infos)
}
