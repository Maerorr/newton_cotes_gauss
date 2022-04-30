use crate::hermite_gauss::hermite_gauss;
use eframe::{
    egui::{
        self,
        plot::{Line, Plot, PlotImage, PlotUi, Points, VLine, Value, Values},
        Image, Layout, CentralPanel,
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

struct AppState {
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
    gauss_proper_weight: bool,
}

impl AppState {
    fn new() -> AppState {
        AppState {
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
            gauss_proper_weight: false,
        }
    }
    fn default(&mut self) {
        self.left = -2.;
        self.right = 2.;
        self.function = Function::Poly1;
        self.eps = 0.001;
        self.output_text = String::new();
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, frame: &eframe::epi::Frame) {
        ctx.set_pixels_per_point(1.5);
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Numerical Integration");

            // NEWTON COTES GROUP

            ui.group(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Newton-Cotes");
                    ui.add_space(20.);
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
                            egui::Slider::new(&mut self.eps, 1e-10..=0.1).logarithmic(true),
                        );
                    });
                });
            });

            // GAUSS HERMITE GROUP

            ui.group(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Gauss-Hermite");
                    ui.add_space(20.);
                    ui.horizontal(|ui| {
                        ui.label("No. of Nodes:");
                        ui.add(
                            egui::Slider::new(&mut self.gauss_nodes, 2..=6)
                        );
                    });
                    ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.add_space(15.);
                        ui.heading("Weight");

                        ui.add_space(5.);
                        ui.radio_value(&mut self.gauss_proper_weight, false, "e weight");
                        ui.radio_value(&mut self.gauss_proper_weight, true, "proper weight");
                        ui.add_space(5.);
                    });
                });

            });

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {

                    // FUNCTIONS GROUP

                    ui.group(|ui| {
                        ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.heading("Function");

                            ui.add_space(5.);
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
                            ui.add_space(20.);
                        });

                    });

                    // OUTPUT GROUP

                    ui.group(|ui| {
                        ui.with_layout(Layout::top_down(egui::Align::LEFT), |ui| {
                            ui.heading("Output");

                            ui.add_space(5.);

                            if ui.button("Calculate").clicked() {
                                let out_newton = newton_cotes(self.function, self.left, self.right, self.eps);
                                let out_gauss = hermite_gauss(self.function, self.gauss_nodes, self.gauss_proper_weight);

                                self.output_text = format!(
                                    "Newton-Cotes:\nResult: {:.3}\nIteration nr.: {}\n\nGauss-Hermite:\nResult: {:.3}", out_newton.0, out_newton.1, out_gauss);
                            }
                            ui.heading(&self.output_text);
                            ui.add_space(2.);
                        });
                    });
                });

            });
        });
    }

    fn name(&self) -> &str {
        "Numerical Integration"
    }
}

pub fn main() {
    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480., 800.0)),
        ..eframe::NativeOptions::default()
    };
    run_native(Box::new(AppState::new()), native_options);
}
