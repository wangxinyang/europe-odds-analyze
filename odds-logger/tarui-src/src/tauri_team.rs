use data::{OddsError, Team, TeamBuilder};
use odds::{EuropeOdds, OddsManager};
use tauri::State;

#[tauri::command]
pub async fn get_team_lists(manager: State<'_, OddsManager>) -> Result<Vec<Team>, OddsError> {
    let manager = &*manager;
    let leagues = manager.list_teams().await?;
    Ok(leagues)
}

#[tauri::command]
pub async fn save_team_info(
    manager: State<'_, OddsManager>,
    id: i32,
    name: String,
    note: String,
) -> Result<Vec<Team>, OddsError> {
    let manager = &*manager;
    let team = TeamBuilder::default()
        .league_id(id)
        .name(name)
        .note(note)
        .build()
        .unwrap();
    let teams = manager.create_team(team).await?;
    Ok(teams)
}

#[tauri::command]
pub async fn delete_team_info(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<Vec<Team>, OddsError> {
    let manager = &*manager;
    let teams = manager.delete_team(id).await?;
    Ok(teams)
}
