use data::{League, LeagueBuilder, OddsError};
use odds::{EuropeOdds, OddsManager};
use tauri::State;

#[tauri::command]
pub async fn get_league_lists(manager: State<'_, OddsManager>) -> Result<Vec<League>, OddsError> {
    let manager = &*manager;
    let leagues = manager.list_leagues().await?;
    Ok(leagues)
}

#[tauri::command]
pub async fn save_league_info(
    manager: State<'_, OddsManager>,
    name: String,
    note: String,
) -> Result<Vec<League>, OddsError> {
    let manager = &*manager;
    let league = LeagueBuilder::default()
        .name(name)
        .note(note)
        .build()
        .unwrap();
    let leagues = manager.create_league(league).await?;
    Ok(leagues)
}

#[tauri::command]
pub async fn delete_league_info(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<Vec<League>, OddsError> {
    let manager = &*manager;
    let leagues = manager.delete_league(id).await?;
    Ok(leagues)
}

#[tauri::command]
pub async fn get_league_with_id(
    manager: State<'_, OddsManager>,
    id: i32,
) -> Result<League, OddsError> {
    let manager = &*manager;
    let league = manager.query_league_with_id(id).await?;
    Ok(league)
}

#[tauri::command]
pub async fn update_league_info(
    manager: State<'_, OddsManager>,
    id: i32,
    name: String,
    note: String,
) -> Result<Vec<League>, OddsError> {
    let manager = &*manager;
    let league = LeagueBuilder::default()
        .id(id)
        .name(name)
        .note(note)
        .build()
        .unwrap();
    let leagues = manager.update_league(league).await?;
    Ok(leagues)
}
