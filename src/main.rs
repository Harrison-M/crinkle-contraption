use std::time::{Duration, Instant};

use eframe::{
    egui::{self, Button},
    epaint::{Color32, Rounding, Stroke},
};

/// Maximum number of orbs
const ORB_COUNT: f32 = 6.0;
/// Maximum fill level of the meter
const METER_MAX: f32 = 700.0;
/// Amount of the meter that should be filled in the "expected turn time"
const DEFAULT_METER_PER_TURN: f32 = 200.0;

// The villain of the one-shot is named Bertram Crinkle

/// Everything needed to represent Bertram Crinkle's dread machine
struct CrinkleContraption {
    charge: f32,
    running: bool,
    last_frame: Instant,
    orbs: [bool; 6],
    turn_length: Duration,
    calibrating: bool,
    calibration_start: Instant,
    difficulty: f32,
}

impl Default for CrinkleContraption {
    fn default() -> Self {
        Self {
            charge: 0.0,
            running: false,
            last_frame: Instant::now(),
            orbs: [true; 6],
            turn_length: Duration::from_secs(10),
            calibrating: false,
            calibration_start: Instant::now(),
            difficulty: DEFAULT_METER_PER_TURN,
        }
    }
}

impl CrinkleContraption {
    /// Create the contraption
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self::default()
    }
}

impl eframe::App for CrinkleContraption {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let new_time = Instant::now();
        let active_orbs = self.orbs.iter().filter(|&&o| o).count() as f32;
        if self.running && active_orbs > 0.0 {
            let time_since_last_draw = new_time - self.last_frame;
            let broken_orbs: f32 = ORB_COUNT - active_orbs;
            self.charge = METER_MAX.min(
                self.charge
                    + time_since_last_draw.as_secs_f32() / self.turn_length.as_secs_f32()
                        * self.difficulty
                        / (broken_orbs + 1.0),
            );
        }
        self.last_frame = new_time;

        egui::TopBottomPanel::bottom("Controls").show(ctx, |ui| {
            // Timer controls
            ui.horizontal(|ui| {
                if self.running {
                    if ui.button("Pause").clicked() {
                        self.running = false;
                    }
                } else if ui.button("Run").clicked() {
                    self.running = true;
                }

                if ui.button("Reset").clicked() {
                    self.charge = 0.0;
                }

                ui.label(format!("Charge: {:.2}", self.charge));
            });

            // Orb status
            ui.horizontal(|ui| {
                for i in 0..6 {
                    ui.add(egui::Checkbox::new(&mut self.orbs[i], (i + 1).to_string()));
                }
            });

            // Calibration controls
            ui.horizontal(|ui| {
                if self.calibrating {
                    if ui.button("Finish calibration").clicked() {
                        self.calibrating = false;
                        self.turn_length = Instant::now().duration_since(self.calibration_start);
                        println!("Turn length: {:.2}", self.turn_length.as_secs_f32());
                    }
                } else {
                    if ui.button("Start calibration").clicked() {
                        self.calibration_start = Instant::now();
                        self.calibrating = true;
                    }
                }
            });

            // Spellcasting levels
            ui.horizontal(|ui| {
                let width = ui.available_width();

                let button_1 = Button::new("1").min_size(egui::vec2(width / 7.0, 0.0));
                if ui.add_enabled(self.charge >= 100.0, button_1).clicked() {
                    self.charge -= 100.0;
                }
                let button_2 = Button::new("2").min_size(egui::vec2(width / 7.0, 0.0));
                if ui.add_enabled(self.charge >= 200.0, button_2).clicked() {
                    self.charge -= 200.0;
                }
                let button_3 = Button::new("3").min_size(egui::vec2(width / 7.0, 0.0));
                if ui.add_enabled(self.charge >= 300.0, button_3).clicked() {
                    self.charge -= 300.0;
                }
                let button_4 = Button::new("4").min_size(egui::vec2(width / 7.0, 0.0));
                if ui.add_enabled(self.charge >= 400.0, button_4).clicked() {
                    self.charge -= 400.0;
                }
                let button_5 = Button::new("5").min_size(egui::vec2(width / 7.0, 0.0));
                if ui.add_enabled(self.charge >= 500.0, button_5).clicked() {
                    self.charge -= 500.0;
                }
                let button_6 = Button::new("6").min_size(egui::vec2(width / 7.0, 0.0));
                if ui.add_enabled(self.charge >= 600.0, button_6).clicked() {
                    self.charge -= 600.0;
                }
            });

            // Adjustments
            ui.horizontal(|ui| {
                if ui.button("-50").clicked() {
                    self.charge = (self.charge - 50.0).max(0.0);
                }
                if ui.button("+50").clicked() {
                    self.charge = (self.charge + 50.0).min(METER_MAX);
                }
                if ui.button("Dif-").clicked() {
                    self.difficulty = (self.difficulty - 50.0).max(0.0);
                }
                if ui.button("Dif+").clicked() {
                    self.difficulty = self.difficulty + 50.0;
                }

                ui.label(format!("Dif: {:.2}", self.difficulty));
            });
        });

        // Draw the meter
        egui::CentralPanel::default().show(ctx, |ui| {
            let width = ui.available_width();
            let height = ui.available_height();

            let quarter_width = width / 4.0;
            let painter = ui.painter();
            painter.rect_stroke(
                eframe::epaint::Rect {
                    min: egui::pos2(quarter_width, 10.0),
                    max: egui::pos2(width - quarter_width, height - 1.0),
                },
                Rounding::default(),
                Stroke {
                    color: Color32::WHITE,
                    width: 1.0,
                },
            );
            painter.rect_filled(
                eframe::epaint::Rect {
                    min: egui::pos2(
                        quarter_width + 1.0,
                        11.0 + (METER_MAX - self.charge) / METER_MAX * (height - 11.0),
                    ),
                    max: egui::pos2(width - quarter_width - 1.0, height - 2.0),
                },
                Rounding::default(),
                Color32::from_rgb(255, 0, 255),
            );
        });

        // Run at roughly 12 fps
        ctx.request_repaint_after(Duration::from_secs_f32(1.0 / 12.0));
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(300.0, 720.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Crinkle Contraption",
        options,
        Box::new(|cc| Box::new(CrinkleContraption::new(cc))),
    );
}
