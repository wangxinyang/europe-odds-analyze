use egui::Ui;

#[derive(Default)]
pub struct BookMakers {
    name: String,
    url: String,
    note: String,
}

impl BookMakers {
    pub fn ui(&mut self, ui: &mut Ui) {
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

        if ui.button("save").clicked() {
            println!("save, {}", self.name);
        }
    }
}
