use presentation::EuroOddsRecoder;

#[tokio::main]
async fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        transparent: true,
        fullscreen: false,
        resizable: false,
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Odds Recorder",
        options,
        Box::new(|_cc| Box::new(EuroOddsRecoder::default())),
    )
}
