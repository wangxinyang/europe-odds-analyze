use egui::{Color32, RichText, Ui};
use egui_extras::Column;
use odds::{EuropeOdds, OddsManager};
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use crate::{initial_strip_layout, initial_table_layout};
use data::{League, LeagueBuilder, OddsError};

struct LeaguesTable {
    list: Vec<League>,
    striped: bool,
}

struct LeaguesFormInit {
    first_load: bool,
    open: bool,
    err: String,
    manager: Arc<OddsManager>,
}

struct LeaguesForm {
    name: String,
    note: String,
    table: LeaguesTable,
}

struct LeaguesChannel {
    tx: Sender<Vec<League>>,
    rx: Receiver<Vec<League>>,
    error_tx: Sender<OddsError>,
    error_rx: Receiver<OddsError>,
}

pub struct Leagues {
    init: LeaguesFormInit,
    form: LeaguesForm,
    channel: LeaguesChannel,
}

impl Leagues {
    pub fn new(manager: Arc<OddsManager>) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let (error_tx, error_rx) = std::sync::mpsc::channel();
        Self {
            init: LeaguesFormInit {
                first_load: true,
                open: false,
                err: Default::default(),
                manager,
            },
            form: LeaguesForm {
                name: Default::default(),
                note: Default::default(),
                table: LeaguesTable {
                    striped: true,
                    list: vec![],
                },
            },
            channel: LeaguesChannel {
                tx,
                rx,
                error_tx,
                error_rx,
            },
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        // init the data
        if self.init.first_load {
            self.init.first_load = false;
            get_all_lists(
                self.init.manager.clone(),
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

        // title
        ui.vertical_centered(|ui| {
            ui.heading("🎨 Leagues Settings");
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
        self.integrate_input_form(ui, self.init.manager.clone());

        ui.add_space(100.);

        ui.separator();
        // data list
        initial_strip_layout(ui, |ui| self.table_ui(ui));
    }

    /// integrate_input_form
    fn integrate_input_form(&mut self, ui: &mut Ui, manager: Arc<OddsManager>) {
        ui.horizontal(|ui| {
            ui.label("公司名称:");
            ui.label(RichText::new("*").color(Color32::RED));
            ui.text_edit_singleline(&mut self.form.name)
                .on_hover_text("请输入联赛名称,必须填写");
        });

        ui.separator();
        ui.add_space(20.);

        ui.horizontal(|ui| {
            ui.label("备注:");
            ui.add_space(38.);
            ui.text_edit_singleline(&mut self.form.note)
                .on_hover_text("请输入备注信息");
        });

        let league = LeagueBuilder::default()
            .name(self.form.name.clone())
            .note(self.form.note.clone())
            .build()
            .unwrap();

        ui.separator();
        ui.add_space(58.);

        ui.horizontal(|ui| {
            ui.add_space(320.);

            // query button
            if ui
                .button(RichText::new("查询").color(Color32::RED).size(15.0))
                .clicked()
            {
                get_all_lists(
                    manager.clone(),
                    self.channel.tx.clone(),
                    self.channel.error_tx.clone(),
                    ui.ctx().clone(),
                );
            }

            // save button
            if ui
                .button(RichText::new("保存").color(Color32::RED).size(15.0))
                .clicked()
            {
                if self.form.name.is_empty() {
                    self.init.err = "请输入联赛名称".into();
                    self.init.open = true;
                } else {
                    save_league_form(
                        manager,
                        league,
                        self.channel.tx.clone(),
                        self.channel.error_tx.clone(),
                        ui.ctx().clone(),
                    );
                    // clear input
                    self.form.name = Default::default();
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
                        ui.strong("联赛名称").on_hover_text("联赛名称");
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
                    body.row(30., |mut row| {
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
                                ui.label(bms.note.clone().unwrap())
                                    .on_hover_text(bms.note.clone().unwrap());
                            });
                        });

                        row.col(|ui| {
                            // update bookmaker info
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

                            // delete bookmaker info
                            if ui
                                .button(RichText::new("删除").color(Color32::RED).size(15.))
                                .clicked()
                            {
                                delete(
                                    self.init.manager.clone(),
                                    bms.id,
                                    self.channel.tx.clone(),
                                    self.channel.error_tx.clone(),
                                    ui.ctx().clone(),
                                );
                            }
                        });
                    });
                }
            });
    }
}

/// get bookmakers
fn get_all_lists(
    odds_manager: Arc<OddsManager>,
    tx: Sender<Vec<League>>,
    err_tx: Sender<OddsError>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        match odds_manager.list_leagues().await {
            Ok(leagues) => {
                tx.send(leagues).unwrap();
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
fn save_league_form(
    odds_manager: Arc<OddsManager>,
    league: League,
    tx: Sender<Vec<League>>,
    err_tx: Sender<OddsError>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        match odds_manager.create_league(league).await {
            Ok(leagues) => {
                tx.send(leagues).unwrap();
                ctx.request_repaint();
            }
            Err(err) => {
                err_tx.send(err).unwrap();
                ctx.request_repaint();
            }
        }
    });
}

/// delete the bookmaker info
fn delete(
    odds_manager: Arc<OddsManager>,
    id: i32,
    tx: Sender<Vec<League>>,
    err_tx: Sender<OddsError>,
    ctx: egui::Context,
) {
    tokio::spawn(async move {
        match odds_manager.delete_league(id).await {
            Ok(leagues) => {
                tx.send(leagues).unwrap();
                ctx.request_repaint();
            }
            Err(err) => {
                err_tx.send(err).unwrap();
                ctx.request_repaint();
            }
        }
    });
}
