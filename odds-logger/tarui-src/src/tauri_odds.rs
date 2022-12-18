use data::{OddsError, Team};
use odds::{EuropeOdds, OddsManager};
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
