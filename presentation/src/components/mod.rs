mod bookmakers;
mod leagues;
mod wrap_app;

use egui::{Ui, Window};

pub use bookmakers::*;
pub use leagues::*;
pub use wrap_app::*;

#[derive(Default)]
pub struct MessageInfo {
    show_window: bool,
}

impl MessageInfo {
    pub fn name(&self) -> &'static str {
        "Message Information"
    }

    pub fn show(&mut self, ui: &mut Ui, open: &mut bool, content: impl Into<String>) {
        Window::new(self.name())
            .open(open)
            .show(ui.ctx(), |ui| self.ui(ui, content));
    }

    pub fn ui(&mut self, ui: &mut Ui, content: impl Into<String>) {
        ui.label(content.into());
    }
}
