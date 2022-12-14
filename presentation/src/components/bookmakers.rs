use std::sync::mpsc::{Receiver, Sender};

use data::{BookMaker, BookMakerBuilder, Config};
use egui::Ui;
use odds::{EuropeOdds, OddsManager};

// #[derive(Default)]
pub struct BookMakers {
    name: String,
    url: String,
    note: String,
    tx: Sender<BookMaker>,
    rx: Receiver<BookMaker>,
}

impl Default for BookMakers {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        Self {
            name: String::new(),
            url: String::new(),
            note: String::new(),
            tx,
            rx,
        }
    }
}

impl BookMakers {
    pub fn ui(&mut self, ui: &mut Ui) {
        if let Ok(bm) = self.rx.try_recv() {
            self.name = bm.name;
        }

        ui.label("bookmaker settings");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("bookmaker name:");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("bookmaker url:");
            ui.text_edit_singleline(&mut self.url);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("bookmaker note:");
            ui.text_edit_singleline(&mut self.note);
        });

        // if ui.button("save").clicked() {
        //     println!("save, {}", self.name);
        // }

        if ui.button("保存数据").clicked() {
            get_bookmakers(self.tx.clone(), ui.ctx().clone());
        }
    }
}

fn get_bookmakers(tx: Sender<BookMaker>, ctx: egui::Context) {
    tokio::spawn(async move {
        let config = Config::from_file("./odds.yml").expect("Unable to parse file");
        let odds_manager = OddsManager::from_config(&config.db)
            .await
            .expect("Unable to get OddsManager");
        let book_maker = BookMakerBuilder::default()
            .name("test")
            .url("test")
            .build()
            .unwrap();
        let bookmakers = odds_manager.create_bookermaker(book_maker).await.unwrap();
        tx.send(bookmakers).unwrap();
        ctx.request_repaint();
    });
}
