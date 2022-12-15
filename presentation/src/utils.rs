use egui::containers::Frame;
use egui::style::Margin;
use egui::{Color32, FontData, FontDefinitions, FontFamily, Ui};
use egui_extras::{Size, StripBuilder, TableBuilder};

/// define my font
pub fn define_my_font() -> FontDefinitions {
    let mut fonts = FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters):
    fonts.font_data.insert(
        "my_font".to_owned(),
        FontData::from_static(include_bytes!("./fonts/NotoSansSC-Regular.otf")),
    ); // .ttf and .otf supported

    // Put my font first (highest priority):
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .push("my_font".to_owned());

    fonts
    // egui_ctx.set_fonts(fonts);
}

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
}

/// initial central panel frame
pub fn initial_central_panel_frame() -> Frame {
    Frame {
        inner_margin: Margin {
            left: 30.,
            right: 30.,
            top: 10.,
            bottom: 10.,
        },
        outer_margin: Margin {
            left: 0.,
            right: 0.,
            top: 0.,
            bottom: 0.,
        },
        rounding: egui::Rounding {
            nw: 1.0,
            ne: 1.0,
            sw: 1.0,
            se: 1.0,
        },
        shadow: eframe::epaint::Shadow {
            extrusion: 1.0,
            color: Color32::BROWN,
        },
        fill: Color32::DARK_GRAY,
        stroke: egui::Stroke::new(2.0, Color32::GRAY),
    }
}
