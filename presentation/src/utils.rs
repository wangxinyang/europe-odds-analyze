use egui::Ui;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};

/// Strip layout initial
pub fn initial_strip_layout(ui: &mut Ui, f: impl FnOnce(&mut Ui)) {
    StripBuilder::new(ui)
        .size(Size::remainder().at_least(100.0)) // for the table
        .size(Size::exact(10.0)) // for the source code link
        .vertical(|mut strip| {
            strip.cell(|ui| {
                egui::ScrollArea::horizontal().show(ui, f);
            });
        });
}

/// Table layout initial
pub fn initial_table_layout(ui: &mut egui::Ui, is_striped: bool) -> TableBuilder {
    TableBuilder::new(ui)
        .striped(is_striped) // add stripes to the table for easy reading
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .column(Column::initial(200.0).range(40.0..=300.0).resizable(true))
        .column(
            Column::initial(100.0)
                .at_least(100.0)
                .resizable(true)
                .clip(true),
        )
        .column(Column::remainder())
        .min_scrolled_height(0.0)
}
