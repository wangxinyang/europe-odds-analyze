use presentation::EuroOddsRecoder;

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        maximized: true,
        ..Default::default()
    };
    eframe::run_native(
        "Odds Recorder",
        options,
        Box::new(|_cc| Box::new(EuroOddsRecoder::default())),
    )
}
