use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use data::{BookMaker, BookMakerBuilder, OddsError};
use egui::Ui;
use odds::{EuropeOdds, OddsManager};

pub struct BookMakers {
    open: bool,
    err: String,
    name: String,
    url: String,
    note: String,
    tx: Sender<BookMaker>,
    rx: Receiver<BookMaker>,
    error_tx: Sender<OddsError>,
    error_rx: Receiver<OddsError>,
}

impl Default for BookMakers {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (error_tx, error_rx) = std::sync::mpsc::channel();
        Self {
            open: false,
            err: Default::default(),
            name: Default::default(),
            url: Default::default(),
            note: Default::default(),
            tx,
            rx,
            error_tx,
            error_rx,
        }
    }
}

impl BookMakers {
    pub fn ui(&mut self, ui: &mut Ui, manager: Arc<OddsManager>) {
        if let Ok(bm) = self.rx.try_recv() {
            // TODO: update ui
            self.name = bm.name;
        }
        if let Ok(error) = self.error_rx.try_recv() {
            self.err = error.to_string();
            self.open = true;
        }

        ui.label("bookmaker settings");
        ui.separator();

        // Error message window show or close
        egui::Window::new("Error")
            .open(&mut self.open.clone())
            .show(ui.ctx(), |ui| {
                ui.label(self.err.clone());
                if ui.button("OK").clicked() {
                    self.open = false;
                }
            });

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Url:");
            ui.text_edit_singleline(&mut self.url);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Note:");
            ui.text_edit_singleline(&mut self.note);
        });

        let book_maker = BookMakerBuilder::default()
            .name(self.name.clone())
            .url(self.url.clone())
            .note(self.note.clone())
            .build()
            .unwrap();

        ui.separator();

        if ui.button("save").clicked() {
            save_bookmaker(
                manager,
                book_maker,
                self.tx.clone(),
                self.error_tx.clone(),
                ui.ctx().clone(),
            );
            // clear input
            self.name = Default::default();
            self.url = Default::default();
            self.note = Default::default();
        }

        ui.separator();

        egui::Grid::new("some_unique_id").show(ui, |ui| {
            ui.label("First row, first column");
            ui.label("First row, second column");
            ui.end_row();

            ui.label("Second row, first column");
            ui.label("Second row, second column");
            ui.label("Second row, third column");
            ui.end_row();

            ui.horizontal(|ui| {
                ui.label("Same");
                ui.label("cell");
            });
            ui.label("Third row, second column");
            ui.end_row();
        });
    }
}

/// save the bookmaker info
fn save_bookmaker(
    odds_manager: Arc<OddsManager>,
    book_maker: BookMaker,
    tx: Sender<BookMaker>,
    err_tx: Sender<OddsError>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        match odds_manager.create_bookermaker(book_maker).await {
            Ok(bookmakers) => {
                tx.send(bookmakers).unwrap();
                ctx.request_repaint();
            }
            Err(err) => {
                err_tx.send(err).unwrap();
                ctx.request_repaint();
            }
        }
    });
}
