use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use data::{BookMaker, BookMakerBuilder, OddsError};
use egui::Ui;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use odds::{EuropeOdds, OddsManager};

pub struct TableDemo {
    list: Vec<BookMaker>,
    striped: bool,
}

pub struct BookMakers {
    first_load: bool,
    open: bool,
    err: String,
    name: String,
    url: String,
    note: String,
    table: TableDemo,
    tx: Sender<Vec<BookMaker>>,
    rx: Receiver<Vec<BookMaker>>,
    error_tx: Sender<OddsError>,
    error_rx: Receiver<OddsError>,
}

impl Default for BookMakers {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (error_tx, error_rx) = std::sync::mpsc::channel();
        Self {
            first_load: true,
            open: false,
            err: Default::default(),
            name: Default::default(),
            url: Default::default(),
            note: Default::default(),
            table: TableDemo {
                striped: true,
                list: vec![],
            },
            tx,
            rx,
            error_tx,
            error_rx,
        }
    }
}

impl BookMakers {
    pub fn ui(&mut self, ui: &mut Ui, manager: Arc<OddsManager>) {
        if self.first_load {
            self.first_load = false;
            list_bookmaker(
                manager.clone(),
                self.tx.clone(),
                self.error_tx.clone(),
                ui.ctx().clone(),
            );
        }

        if let Ok(bm) = self.rx.try_recv() {
            self.table.list = bm;
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

        StripBuilder::new(ui)
            .size(Size::remainder().at_least(100.0)) // for the table
            .size(Size::exact(10.0)) // for the source code link
            .vertical(|mut strip| {
                strip.cell(|ui| {
                    egui::ScrollArea::horizontal().show(ui, |ui| {
                        self.table_ui(ui);
                    });
                });
            });
    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
        let table = TableBuilder::new(ui)
            .striped(self.table.striped)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::initial(100.0).resizable(false))
            // .column(Column::initial(200.0).range(40.0..=300.0).resizable(true))
            .column(
                Column::initial(40.0)
                    .at_least(40.0)
                    .resizable(false)
                    .clip(true),
            )
            .column(Column::remainder())
            .min_scrolled_height(0.0);

        // if let Some(row_nr) = self.table.scroll_to_row.take() {
        //     table = table.scroll_to_row(row_nr, None);
        // }

        table
            .header(50.0, |mut header| {
                header.col(|ui| {
                    ui.strong("No.");
                });
                header.col(|ui| {
                    ui.strong("Name");
                });
                header.col(|ui| {
                    ui.strong("Url");
                });
                header.col(|ui| {
                    ui.strong("Note");
                });
            })
            .body(|mut body| {
                for (index, bms) in self.table.list.iter().enumerate() {
                    // let is_thick = thick_row(row_index);
                    let row_height = 30.0;
                    // let row_height = if is_thick { 30.0 } else { 18.0 };
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.label(index.to_string());
                        });
                        row.col(|ui| {
                            // expanding_content(ui);
                            ui.label(bms.name.clone());
                        });
                        row.col(|ui| {
                            ui.label(bms.url.clone().unwrap());
                        });
                        row.col(|ui| {
                            ui.label(bms.note.clone().unwrap());
                        });
                    });
                }
            });
    }
}

/// get bookmakers
fn list_bookmaker(
    odds_manager: Arc<OddsManager>,
    tx: Sender<Vec<BookMaker>>,
    err_tx: Sender<OddsError>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        match odds_manager.list_bookermaker().await {
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

/// save the bookmaker info
fn save_bookmaker(
    odds_manager: Arc<OddsManager>,
    book_maker: BookMaker,
    tx: Sender<Vec<BookMaker>>,
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
