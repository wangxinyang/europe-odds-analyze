#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use tokio::sync::mpsc;

use app::{
    __cmd__delete_book_maker_info, __cmd__delete_league_info, __cmd__get_book_maker_lists,
    __cmd__get_league_lists, __cmd__save_book_maker_info, __cmd__save_league_info,
    delete_book_maker_info, delete_league_info, get_book_maker_lists, get_league_lists,
    save_book_maker_info, save_league_info,
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
        ])
        .setup(|app| {
            app.manage(odds_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
