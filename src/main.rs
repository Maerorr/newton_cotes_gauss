use crate::hermite_gauss::hermite_gauss;
use eframe::{
    egui::{
        self,
        plot::{Line, Plot, PlotImage, PlotUi, Points, VLine, Value, Values},
        Image, Layout,
    },
    epaint::{vec2, Color32, TextureId},
    epi::App,
    run_native,
};
use functions::{function_value, weight};
use newton_cotes::newton_cotes;

mod functions;
mod hermite_gauss;
mod newton_cotes;

#[derive(PartialEq, Clone, Copy)]
pub enum Function {
    Poly1,
    Poly2,
    Linear,
    Sinusoidal,
    Absolute,
    Mixed,
}

enum OperationMode {
    None,
    Newton,
    Gauss,
}

struct AppState {
    mode: OperationMode,
    left_not_plot: f64,
    right_not_plot: f64,
    left: f64,
    right: f64,
    function: Function,
    eps: f64,
    output_text: String,
    plot_left: Vec<Value>,
    plot_middle: Vec<Value>,
    plot_right: Vec<Value>,
    gauss_nodes: usize,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            mode: OperationMode::None,
            left: -2.,
            right: 2.,
            function: Function::Poly1,
            eps: 0.001,
            output_text: String::new(),
            plot_left: vec![Value::new(0., 0.)],
            plot_middle: vec![Value::new(0., 0.)],
            plot_right: vec![Value::new(0., 0.)],
            left_not_plot: -2.,
            right_not_plot: 2.,
            gauss_nodes: 3,
        }
    }
    fn default(&mut self) {
        self.mode = OperationMode::None;
        self.left = -2.;
        self.right = 2.;
        self.function = Function::Poly1;
        self.eps = 0.001;
        self.output_text = String::new();
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &eframe::epi::Frame) {
        ctx.set_pixels_per_point(1.5);
        match self.mode {
            // DEFAULT WINDOW
            OperationMode::None => {
                egui::SidePanel::left("left_panel")
                    .min_width(150.)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading("Choose Operation Mode");
                            ui.add_space(5.0);
                            if ui.button("Newton").clicked() {
                                self.mode = OperationMode::Newton;
                            };
                            ui.add_space(5.0);
                            if ui.button("Gauss").clicked() {
                                self.mode = OperationMode::Gauss;
                            };
                        })
                    });
            }

            // NEWTON WINDOW
            OperationMode::Newton => {
                egui::SidePanel::left("left_panel")
                    .min_width(150.)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading("Newton-Cotes Method");

                            ui.heading("Function");
                            ui.add_space(5.);

                            ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                                ui.radio_value(
                                    &mut self.function,
                                    Function::Poly1,
                                    "Polynomial 2nd Power",
                                );
                                ui.radio_value(
                                    &mut self.function,
                                    Function::Poly2,
                                    "Polynomial 4th power",
                                );
                                ui.radio_value(&mut self.function, Function::Linear, "Linear");
                                ui.radio_value(
                                    &mut self.function,
                                    Function::Sinusoidal,
                                    "Sinusoidal",
                                );
                                ui.radio_value(&mut self.function, Function::Absolute, "Absolute");
                                ui.radio_value(&mut self.function, Function::Mixed, "Mixed");
                            });

                            ui.label("Integration Range");
                            ui.horizontal(|ui| {
                                ui.label("Left:");
                                ui.add(egui::DragValue::new(&mut self.left_not_plot));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Right:");
                                ui.add(egui::DragValue::new(&mut self.right_not_plot));
                            });

                            ui.horizontal(|ui| {
                                ui.label("Epsilon:");
                                ui.add(
                                    egui::Slider::new(&mut self.eps, 1e-20..=0.1).logarithmic(true),
                                );
                            });

                            // CALCULATIONS

                            if ui.button("Calculate").clicked() {
                                self.output_text.clear();

                                // we're using separate variables for left and right so the lines stay in place
                                // while moving the sliders
                                self.left = self.left_not_plot;
                                self.right = self.right_not_plot;

                                let out =
                                    newton_cotes(self.function, self.left, self.right, self.eps);

                                self.output_text.push_str(
                                    format!("{:.3}, in {} iterations", out.0, out.1).as_str(),
                                );

                                // generation of points for the plot

                                // left side no fill
                                let mut vec: Vec<Value> = Vec::new();
                                for i in 0..1000 {
                                    let x = self.left - 5.
                                        + i as f64 * ((self.left) - (self.left - 5.)) / 1000.;
                                    vec.push(Value::new(x, function_value(x, self.function)));
                                }
                                self.plot_left = vec;

                                // middle side with fill
                                let mut vec: Vec<Value> = Vec::new();
                                for i in 0..10000 {
                                    let x = self.left
                                        + i as f64 * ((self.right) - (self.left)) / 10000.;
                                    vec.push(Value::new(x, function_value(x, self.function)));
                                }
                                self.plot_middle = vec;

                                // right side no fill
                                let mut vec: Vec<Value> = Vec::new();
                                for i in 0..1000 {
                                    let x = self.right
                                        + i as f64 * ((self.right + 5.) - (self.right)) / 1000.;
                                    vec.push(Value::new(x, function_value(x, self.function)));
                                }
                                self.plot_right = vec;
                            }

                            // CALCULATIONS END

                            ui.label(self.output_text.as_str());

                            ui.add_space(50.0);
                            if ui.button("Back").clicked() {
                                self.mode = OperationMode::None;
                            };
                        })
                    });
            }

            // GAUSS WINDOW
            OperationMode::Gauss => {
                egui::SidePanel::left("left_panel")
                    .min_width(150.)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading("Gauss Method");

                            ui.heading("Function");
                            ui.add_space(5.);

                            ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                                ui.radio_value(
                                    &mut self.function,
                                    Function::Poly1,
                                    "Polynomial 2nd Power",
                                );
                                ui.radio_value(
                                    &mut self.function,
                                    Function::Poly2,
                                    "Polynomial 4th power",
                                );
                                ui.radio_value(&mut self.function, Function::Linear, "Linear");
                                ui.radio_value(
                                    &mut self.function,
                                    Function::Sinusoidal,
                                    "Sinusoidal",
                                );
                                ui.radio_value(&mut self.function, Function::Absolute, "Absolute");
                                ui.radio_value(&mut self.function, Function::Mixed, "Mixed");
                            });

                            ui.label("Integration Range");
                            ui.horizontal(|ui| {
                                ui.label("Left:");
                                ui.add(egui::DragValue::new(&mut self.left_not_plot));
                            });
                            ui.horizontal(|ui| {
                                ui.label("Right:");
                                ui.add(egui::DragValue::new(&mut self.right_not_plot));
                            });

                            ui.horizontal(|ui| {
                                ui.label("Epsilon:");
                                ui.add(
                                    egui::Slider::new(&mut self.eps, 1e-20..=0.1).logarithmic(true),
                                );
                            });

                            ui.horizontal(|ui| {
                                ui.label("No. of Nodes:");
                                ui.add(
                                    egui::Slider::new(&mut self.gauss_nodes, 2..=6).logarithmic(true),
                                );
                            });
                            ui.label(self.gauss_nodes.to_string().as_str());

                            if ui.button("Calculate").clicked() {
                                self.output_text.clear();

                                // we're using separate variables for left and right so the lines stay in place
                                // while moving the sliders
                                self.left = self.left_not_plot;
                                self.right = self.right_not_plot;

                                let out = hermite_gauss(self.function, self.gauss_nodes);



                                self.output_text.push_str(format!("{:.3}", out).as_str());

                                // generation of points for the plot

                                // left side no fill
                                let mut vec: Vec<Value> = Vec::new();
                                for i in 0..1000 {
                                    let x = self.left - 5.
                                        + i as f64 * ((self.left) - (self.left - 5.)) / 1000.;
                                    vec.push(Value::new(x, function_value(x, self.function)*weight(x)));
                                }
                                self.plot_left = vec;

                                // middle side with fill
                                let mut vec: Vec<Value> = Vec::new();
                                for i in 0..10000 {
                                    let x = self.left
                                        + i as f64 * ((self.right) - (self.left)) / 10000.;
                                    vec.push(Value::new(x, function_value(x, self.function)*weight(x)));
                                }
                                self.plot_middle = vec;

                                // right side no fill
                                let mut vec: Vec<Value> = Vec::new();
                                for i in 0..1000 {
                                    let x = self.right
                                        + i as f64 * ((self.right + 5.) - (self.right)) / 1000.;
                                    vec.push(Value::new(x, function_value(x, self.function)*weight(x)));
                                }
                                self.plot_right = vec;
                            }

                            ui.label(self.output_text.as_str());

                            ui.add_space(5.0);
                            if ui.button("Back").clicked() {
                                self.mode = OperationMode::None;
                            };

                        })
                    });
            }
        }

        // PLOT AREA

        egui::CentralPanel::default().show(ctx, |ui| {
            let f_left = Values::from_values(self.plot_left.clone());
            let f_mid = Values::from_values(self.plot_middle.clone());
            let f_right = Values::from_values(self.plot_right.clone());

            let f_plot_left = Line::new(f_left)
                .name("plot")
                .color(Color32::from_rgb(100, 150, 250));
            let f_plot_mid = Line::new(f_mid)
                .name("plot")
                .fill(0.)
                .color(Color32::from_rgb(100, 150, 250));
            let f_plot_right = Line::new(f_right)
                .name("plot")
                .color(Color32::from_rgb(100, 150, 250));
            let mut plot = Plot::new("plot").legend(egui::widgets::plot::Legend::default());
            let left = VLine::new(self.left)
                .color(Color32::from_rgb(10, 77, 154))
                .name("Integral Range");
            let right = VLine::new(self.right)
                .color(Color32::from_rgb(10, 77, 154))
                .name("Integral Range");

            plot = plot.data_aspect(1.0);
            plot.show(ui, |plot_ui| {
                plot_ui.line(f_plot_left);
                plot_ui.line(f_plot_mid);
                plot_ui.line(f_plot_right);
                plot_ui.vline(left);
                plot_ui.vline(right);
            });
        });
    }

    fn name(&self) -> &str {
        "Numeric Integration"
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200., 660.0)),
        ..eframe::NativeOptions::default()
    };
    run_native(Box::new(AppState::new()), native_options);
}
