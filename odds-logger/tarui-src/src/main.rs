#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use tokio::sync::mpsc;

use app::{
    __cmd__delete_book_maker_info, __cmd__delete_league_info, __cmd__delete_team_info,
    __cmd__get_book_maker_lists, __cmd__get_league_lists, __cmd__get_team_lists,
    __cmd__query_match_info, __cmd__query_teams_with_league, __cmd__save_book_maker_info,
    __cmd__save_league_info, __cmd__save_match_odds, __cmd__save_team_info, delete_book_maker_info,
    delete_league_info, delete_team_info, get_book_maker_lists, get_league_lists, get_team_lists,
    query_match_info, query_teams_with_league, save_book_maker_info, save_league_info,
    save_match_odds, save_team_info,
};
use tauri::async_runtime::block_on;
use tauri::Manager;

use data::Config;
use odds::OddsManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::from_file("./fixtures/config.yml")?;
    let odds_manager = block_on(OddsManager::from_config(&config.db))?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_book_maker_lists,
            save_book_maker_info,
            delete_book_maker_info,
            // legue
            get_league_lists,
            save_league_info,
            delete_league_info,
            // team
            get_team_lists,
            save_team_info,
            delete_team_info,
            // odds
            query_teams_with_league,
            save_match_odds,
            query_match_info,
        ])
        .setup(|app| {
            app.manage(odds_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
