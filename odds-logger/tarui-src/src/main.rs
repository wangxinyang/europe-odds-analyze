#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use tokio::sync::mpsc;

use tauri::async_runtime::block_on;
use tauri::{Manager, State};

use data::{BookMaker, BookMakerBuilder, Config, OddsError};
use odds::{EuropeOdds, OddsManager};

#[tauri::command]
async fn save_book_maker_info(
    manager: State<'_, OddsManager>,
    name: String,
) -> Result<Vec<BookMaker>, OddsError> {
    let manager = &*manager;
    let book_maker = BookMakerBuilder::default()
        .name(name)
        .url("https://www.bet365.com")
        .build()
        .unwrap();
    let bms = manager.create_bookermaker(book_maker).await?;
    Ok(bms)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = Config::from_file("./fixtures/config.yml")?;
    let odds_manager = block_on(OddsManager::from_config(&config.db))?;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_book_maker_info])
        .setup(|app| {
            app.manage(odds_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

// async fn async_process_model(
//     mut input_rx: mpsc::Receiver<String>,
//     output_tx: mpsc::Sender<String>,
// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     loop {
//         while let Some(input) = input_rx.recv().await {
//             let output = input;
//             output_tx.send(output).await?;
//         }
//     }
// }
