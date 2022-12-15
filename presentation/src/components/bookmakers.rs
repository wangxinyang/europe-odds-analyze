use egui::Ui;
use odds::{EuropeOdds, OddsManager};
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use crate::{initial_strip_layout, initial_table_layout};
use data::{BookMaker, BookMakerBuilder, OddsError};

struct BookMakerTable {
    list: Vec<BookMaker>,
    striped: bool,
}

struct BookMakerFormInit {
    first_load: bool,
    open: bool,
    err: String,
}

struct BookMakerForm {
    name: String,
    url: String,
    note: String,
    table: BookMakerTable,
}

struct BookMakerChannel {
    tx: Sender<Vec<BookMaker>>,
    rx: Receiver<Vec<BookMaker>>,
    error_tx: Sender<OddsError>,
    error_rx: Receiver<OddsError>,
}

pub struct BookMakers {
    init: BookMakerFormInit,
    form: BookMakerForm,
    channel: BookMakerChannel,
}

impl Default for BookMakers {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (error_tx, error_rx) = std::sync::mpsc::channel();
        Self {
            init: BookMakerFormInit {
                first_load: true,
                open: false,
                err: Default::default(),
            },
            form: BookMakerForm {
                name: Default::default(),
                url: Default::default(),
                note: Default::default(),
                table: BookMakerTable {
                    striped: true,
                    list: vec![],
                },
            },
            channel: BookMakerChannel {
                tx,
                rx,
                error_tx,
                error_rx,
            },
        }
    }
}

impl BookMakers {
    pub fn ui(&mut self, ui: &mut Ui, manager: Arc<OddsManager>) {
        if self.init.first_load {
            self.init.first_load = false;
            get_book_maker_lists(
                manager.clone(),
                self.channel.tx.clone(),
                self.channel.error_tx.clone(),
                ui.ctx().clone(),
            );
        }

        if let Ok(bm) = self.channel.rx.try_recv() {
            self.form.table.list = bm;
        }
        if let Ok(error) = self.channel.error_rx.try_recv() {
            self.init.err = error.to_string();
            self.init.open = true;
        }

        ui.vertical_centered(|ui| {
            ui.heading("Bookmaker Settings");

            ui.separator();

            // Error message window show or close
            egui::Window::new("Error")
                .open(&mut self.init.open.clone())
                .show(ui.ctx(), |ui| {
                    ui.label(self.init.err.clone());
                    if ui.button("OK").clicked() {
                        self.init.open = false;
                    }
                });

            // generate the input form area
            self.integrate_input_form(ui, manager);

            ui.separator();

            initial_strip_layout(ui, |ui| self.table_ui(ui));
        });
    }

    fn integrate_input_form(&mut self, ui: &mut Ui, manager: Arc<OddsManager>) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.form.name);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Url:");
            ui.text_edit_singleline(&mut self.form.url);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Note:");
            ui.text_edit_singleline(&mut self.form.note);
        });

        let book_maker = BookMakerBuilder::default()
            .name(self.form.name.clone())
            .url(self.form.url.clone())
            .note(self.form.note.clone())
            .build()
            .unwrap();

        ui.separator();

        if ui.button("save").clicked() {
            save_bookmaker_form(
                manager,
                book_maker,
                self.channel.tx.clone(),
                self.channel.error_tx.clone(),
                ui.ctx().clone(),
            );
            // clear input
            self.form.name = Default::default();
            self.form.url = Default::default();
            self.form.note = Default::default();
        }
    }

    fn table_ui(&mut self, ui: &mut egui::Ui) {
        let table = initial_table_layout(ui, self.form.table.striped);

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
                for (index, bms) in self.form.table.list.iter().enumerate() {
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
fn get_book_maker_lists(
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
fn save_bookmaker_form(
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
