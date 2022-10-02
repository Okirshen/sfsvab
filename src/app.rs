use std::ops::RangeInclusive;

use egui::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct App {}

impl Default for App {
    fn default() -> Self {
        Self {}
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        SidePanel::left("Rocket").show(ctx, |ui| {
            ui.label("Rocket");
        });
        SidePanel::right("Properties").show(ctx, |ui| {
            ui.label("Properties");
        });
        CentralPanel::default().show(ctx, |ui| {
            let axis_formatter = |_: f64, _: &RangeInclusive<f64>| -> String { String::new() };
            plot::Plot::new("Graph")
                .x_axis_formatter(axis_formatter)
                .y_axis_formatter(axis_formatter)
                .data_aspect(1.)
                .show(ui, |graph_ui| {});
        });
    }
}
