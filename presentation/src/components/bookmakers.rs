use egui::{Color32, RichText, Ui};
use egui_extras::Column;
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
            ui.heading("📚 Bookmaker Settings");
        });

        ui.add_space(20.);
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

        ui.add_space(100.);

        ui.separator();
        initial_strip_layout(ui, |ui| self.table_ui(ui));
    }

    /// integrate_input_form
    fn integrate_input_form(&mut self, ui: &mut Ui, manager: Arc<OddsManager>) {
        ui.horizontal(|ui| {
            ui.label("公司名称:");
            ui.label(RichText::new("*").color(Color32::RED));
            ui.text_edit_singleline(&mut self.form.name)
                .on_hover_text("请输入公司名称,必须填写");
        });

        ui.separator();
        ui.add_space(20.);

        ui.horizontal(|ui| {
            ui.label("官网:");
            ui.add_space(38.);
            ui.text_edit_singleline(&mut self.form.url)
                .on_hover_text("请输入官网地址");
        });

        ui.separator();
        ui.add_space(20.);

        ui.horizontal(|ui| {
            ui.label("备注:");
            ui.add_space(38.);
            ui.text_edit_singleline(&mut self.form.note)
                .on_hover_text("请输入备注信息");
        });

        let book_maker = BookMakerBuilder::default()
            .name(self.form.name.clone())
            .url(self.form.url.clone())
            .note(self.form.note.clone())
            .build()
            .unwrap();

        ui.separator();
        ui.add_space(58.);

        ui.vertical_centered(|ui| {
            if ui
                .button(RichText::new("保存").color(Color32::RED).size(15.0))
                .clicked()
            {
                if self.form.name.is_empty() {
                    self.init.err = "请输入公司名称".into();
                    self.init.open = true;
                } else {
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
        });

        ui.add_space(8.);
    }

    /// table_ui
    fn table_ui(&mut self, ui: &mut egui::Ui) {
        let table = initial_table_layout(ui, self.form.table.striped);

        table
            .column(Column::auto().resizable(true))
            .column(Column::initial(150.).resizable(true))
            .column(Column::initial(260.).resizable(true))
            .column(Column::initial(150.).resizable(true))
            .column(Column::remainder().resizable(true))
            .min_scrolled_height(0.0)
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.strong("No.").on_hover_text("No.");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.strong("公司名称").on_hover_text("公司名称");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.strong("官网").on_hover_text("官网");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.strong("备注").on_hover_text("备注");
                    });
                });
                header.col(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.strong("操作").on_hover_text("操作");
                    });
                });
            })
            .body(|mut body| {
                for (index, bms) in self.form.table.list.iter().enumerate() {
                    // let is_thick = thick_row(row_index);
                    let row_height = 30.0;
                    // let row_height = if is_thick { 30.0 } else { 18.0 };
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label((index + 1).to_string())
                                    .on_hover_text((index + 1).to_string());
                            });
                        });
                        row.col(|ui| {
                            // expanding_content(ui);
                            ui.vertical_centered(|ui| {
                                ui.label(bms.name.clone()).on_hover_text(bms.name.clone());
                            });
                        });
                        row.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(bms.url.clone().unwrap())
                                    .on_hover_text(bms.url.clone().unwrap());
                            });
                        });
                        row.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(bms.note.clone().unwrap())
                                    .on_hover_text(bms.note.clone().unwrap());
                            });
                        });

                        row.col(|ui| {
                            if ui
                                .button(RichText::new("更新").color(Color32::RED).size(15.))
                                .clicked()
                            {
                                // save_bookmaker_form(
                                //     manager,
                                //     book_maker,
                                //     self.channel.tx.clone(),
                                //     self.channel.error_tx.clone(),
                                //     ui.ctx().clone(),
                                // );
                                println!("update");
                            }
                            // ui.
                            if ui
                                .button(RichText::new("删除").color(Color32::RED).size(15.))
                                .clicked()
                            {
                                // save_bookmaker_form(
                                //     manager,
                                //     book_maker,
                                //     self.channel.tx.clone(),
                                //     self.channel.error_tx.clone(),
                                //     ui.ctx().clone(),
                                // );
                                println!("delete");
                            }
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
