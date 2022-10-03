use std::ops::RangeInclusive;

use egui::*;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct App {
    parts: Vec<Part>,
    is_dragged: bool,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Part {
    pos: Vec2,
    size: Vec2,
}

impl Part {
    fn rect(&self) -> Rect {
        Rect {
            min: self.pos.to_pos2(),
            max: self.size.to_pos2() + self.pos,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            parts: vec![Part {
                pos: vec2(1., 1.),
                size: vec2(1., 1.),
            }],
            is_dragged: false,
        }
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
        let Self { parts, .. } = self;

        SidePanel::left("Rocket").show(ctx, |ui| {
            ui.label("Rocket");
        });
        SidePanel::right("Properties").show(ctx, |ui| {
            ui.label("Properties");
        });
        CentralPanel::default().show(ctx, |ui| {
            let axis_formatter = |_: f64, _: &RangeInclusive<f64>| -> String { String::new() };
            let _grid_spacer = |_input: plot::GridInput| -> Vec<plot::GridMark> {
                let mut marks = vec![];

                marks.push(plot::GridMark {
                    value: 0.,
                    step_size: 1000000.,
                });

                marks
            };
            plot::Plot::new("Graph")
                .x_axis_formatter(axis_formatter)
                .y_axis_formatter(axis_formatter)
                .x_grid_spacer(plot::uniform_grid_spacer(|_| [0.0625, 0.25, 1.]))
                .y_grid_spacer(plot::uniform_grid_spacer(|_| [0.0625, 0.25, 1.]))
                .show_x(false)
                .show_y(false)
                .data_aspect(1.)
                .allow_drag(!self.is_dragged)
                .show(ui, |graph_ui| {
                    graph_ui.hline(plot::HLine::new(0.).width(5.));
                    graph_ui.vline(plot::VLine::new(0.).width(5.));

                    for part in parts {
                        if let Some(pointer) = graph_ui.pointer_coordinate() {
                            if part.rect().contains(pointer.to_pos2()) {
                                let drag = graph_ui.pointer_coordinate_drag_delta();
                                self.is_dragged = drag != Vec2::ZERO;
                                if self.is_dragged {
                                    part.pos += drag;
                                }
                            }
                        } else {
                            self.is_dragged = false;
                        }
                        let Part { pos: p, size: s } = part;
                        graph_ui.polygon(plot::Polygon::new(plot::PlotPoints::new(vec![
                            [p.x.into(), p.y.into()],
                            [p.x.into(), (p.y + s.y).into()],
                            [(p.x + s.x).into(), (p.y + s.y).into()],
                            [(p.x + s.x).into(), p.y.into()],
                        ])));
                    }
                });
        });
    }
}
