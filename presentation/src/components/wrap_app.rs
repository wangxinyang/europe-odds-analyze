use crate::{BookMakers, Leagues};

/// bookmaker app
#[derive(Default)]
pub struct BookMakersApp {
    book_makers: BookMakers,
}

impl eframe::App for BookMakersApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.book_makers.ui(ui);
        });
    }
}
//----------------------------------------------------------------
#[derive(Default)]
pub struct LeaguesApp {
    _league: Leagues,
}

impl eframe::App for LeaguesApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("league");
        });
    }
}

//----------------------------------------------------------------
#[derive(Default)]
pub struct State {
    book_maker: BookMakersApp,
    league: LeaguesApp,

    /// selected anchor
    selected_anchor: String,
}

pub struct EuroOddsRecoder {
    state: State,
}

impl Default for EuroOddsRecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl EuroOddsRecoder {
    pub fn new() -> Self {
        Self {
            state: State::default(),
        }
    }

    fn menu_iter(&mut self) -> impl Iterator<Item = (&str, &str, &mut dyn eframe::App)> {
        vec![
            (
                "📚 Bookmakers",
                "bookmakers",
                &mut self.state.book_maker as &mut dyn eframe::App,
            ),
            (
                "🎨 Leagues",
                "leagues",
                &mut self.state.league as &mut dyn eframe::App,
            ),
        ]
        .into_iter()
    }
}

impl EuroOddsRecoder {
    fn menu_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.separator();

        let mut selected_anchor = self.state.selected_anchor.clone();
        if selected_anchor.is_empty() {
            selected_anchor = "bookmakers".to_string();
        }
        for (name, anchor, _app) in self.menu_iter() {
            if ui
                .selectable_label(selected_anchor == anchor, name)
                .clicked()
            {
                selected_anchor = anchor.to_string();
            }
        }
        self.state.selected_anchor = selected_anchor;
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut found_anchor = false;
        let selected_anchor = self.state.selected_anchor.clone();
        for (_name, anchor, app) in self.menu_iter() {
            if anchor == selected_anchor {
                app.update(ctx, frame);
                found_anchor = true;
            }
        }
        if !found_anchor {
            self.state.selected_anchor = "bookmakers".into();
        }
    }
}

impl eframe::App for EuroOddsRecoder {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::SidePanel::left("menu")
            .max_width(400.0)
            .min_width(400.0)
            .resizable(false)
            .show(ctx, |ui| {
                // add bookmakers
                ui.vertical_centered(|ui| {
                    ui.heading("✒ Menus");
                });

                self.menu_contents(ui, frame);
            });

        self.show_selected_app(ctx, frame);
    }
}
