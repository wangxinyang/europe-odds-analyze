use data::{Config, OddsError};
use odds::OddsManager;
use presentation::EuroOddsRecoder;
use std::sync::mpsc::Sender;

#[tokio::main]
async fn main() -> Result<(), OddsError> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let (tx, rx) = std::sync::mpsc::channel();
    get_odds_manager(tx)?;

    let options = eframe::NativeOptions {
        transparent: true,
        fullscreen: false,
        resizable: false,
        min_window_size: Some(egui::vec2(1400.0, 800.0)),
        initial_window_size: Some(egui::vec2(1400.0, 800.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Odds Recorder",
        options,
        Box::new(|_cc| Box::new(EuroOddsRecoder::new(rx))),
    );

    Ok(())
}

fn get_odds_manager(tx: Sender<OddsManager>) -> Result<(), OddsError> {
    tokio::spawn(async move {
        let config = Config::from_file("./odds.yml")?;
        let odds_manager = OddsManager::from_config(&config.db).await?;
        tx.send(odds_manager).unwrap();
        Ok::<(), OddsError>(())
    });
    Ok(())
}
