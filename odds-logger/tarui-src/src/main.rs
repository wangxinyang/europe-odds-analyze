#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{
    __cmd__delete_book_maker_info, __cmd__delete_league_info, __cmd__delete_match_info,
    __cmd__delete_team_info, __cmd__get_book_maker_lists, __cmd__get_book_maker_with_id,
    __cmd__get_league_lists, __cmd__get_league_with_id, __cmd__get_team_lists,
    __cmd__get_team_with_id, __cmd__query_match_info, __cmd__query_odds_by_id,
    __cmd__query_team_info_by_league, __cmd__save_book_maker_info, __cmd__save_league_info,
    __cmd__save_match_odds, __cmd__save_team_info, __cmd__update_book_maker,
    __cmd__update_league_info, __cmd__update_match_odds, __cmd__update_team_info,
    delete_book_maker_info, delete_league_info, delete_match_info, delete_team_info,
    get_book_maker_lists, get_book_maker_with_id, get_league_lists, get_league_with_id,
    get_team_lists, get_team_with_id, query_match_info, query_odds_by_id,
    query_team_info_by_league, save_book_maker_info, save_league_info, save_match_odds,
    save_team_info, update_book_maker, update_league_info, update_match_odds, update_team_info,
};
use tauri::async_runtime::block_on;
use tauri::Manager;

use data::Config;
use odds::OddsManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_book_maker_lists,
            save_book_maker_info,
            delete_book_maker_info,
            get_book_maker_with_id,
            update_book_maker,
            // legue
            get_league_lists,
            save_league_info,
            delete_league_info,
            get_league_with_id,
            update_league_info,
            // team
            get_team_lists,
            save_team_info,
            delete_team_info,
            get_team_with_id,
            update_team_info,
            query_team_info_by_league,
            // odds
            save_match_odds,
            query_match_info,
            delete_match_info,
            query_odds_by_id,
            update_match_odds,
        ])
        .setup(|app| {
            // Embedding Additional Files with the resource parameter of tauri.conf.json
            let resource_path = app
                .path_resolver()
                .resolve_resource("fixtures/db/config.yml")
                .expect("failed to resolve resource");
            let file = std::fs::File::open(resource_path).unwrap();
            let config = Config::from_file(file)?;
            let odds_manager = block_on(OddsManager::from_config(&config.db))?;
            app.manage(odds_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
