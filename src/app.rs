use std::f64::consts::{PI, TAU};

use crate::pendulum::{DoublePendulum, Pendulum};
use eframe::{
    egui::{self, util::History},
    emath,
};

const MASS_COEFFICIENT: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Pendulum,
    Plots,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Plot {
    Angle,
    Velocity,
    Acceleration,
    Position,
}

pub struct App {
    dp: DoublePendulum,

    time_step: f32,

    running: bool,
    moving: (bool, bool),

    canvas_size: egui::Vec2,
    canvas_transform: emath::RectTransform,

    #[cfg(not(target_arch = "wasm32"))]
    epoch: std::time::Instant,

    position_history: History<(egui::Pos2, egui::Pos2)>,
    angle_history: History<(f32, f32)>,
    velocity_history: History<(f32, f32)>,
    acceleration_history: History<(f32, f32)>,

    current_tab: Tab,
    current_plot: Plot,
}

impl Default for App {
    fn default() -> Self {
        Self {
            dp: DoublePendulum::default(),

            time_step: 0.4,

            running: true,
            moving: (false, false),

            canvas_size: egui::Vec2::ZERO,
            canvas_transform: emath::RectTransform::identity(egui::Rect::ZERO),

            #[cfg(not(target_arch = "wasm32"))]
            epoch: std::time::Instant::now(),

            position_history: History::new(0..10000, 5.0 * 3600.0),
            angle_history: History::new(1..10000, 10.0),
            velocity_history: History::new(1..10000, 10.0),
            acceleration_history: History::new(1..10000, 10.0),

            current_tab: Tab::Pendulum,
            current_plot: Plot::Angle,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.input(ctx);
        self.move_pendula(ctx);

        self.dp.pendula.0.pivot = 0.5 * self.canvas_size.to_pos2();
        self.dp.pendula.1.pivot = self.dp.pendula.0.position();

        if self.running & !self.moving() {
            self.dp.update(self.time_step);
        }

        self.record_history();
        self.ui(ctx);
        ctx.request_repaint();
    }
}

impl App {
    fn input(&mut self, ctx: &egui::Context) {
        ctx.input_mut(|i| {
            if i.key_pressed(egui::Key::Space) {
                self.running = !self.running;
            }

            if i.consume_key(egui::Modifiers::CTRL | egui::Modifiers::ALT, egui::Key::R) {
                self.reset();
            }
        });
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.tabbar(ctx);
        self.settings(ctx);

        match self.current_tab {
            Tab::Pendulum => self.canvas(ctx),
            Tab::Plots => self.plots(ctx),
        }
    }

    fn tabbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::Pendulum,
                    egui::RichText::new("Pendulum").heading(),
                );
                ui.selectable_value(
                    &mut self.current_tab,
                    Tab::Plots,
                    egui::RichText::new("Plots").heading(),
                );
            })
        });
    }

    fn settings(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("settings_panel").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.vertical_centered(|ui| ui.heading("\u{2699} Settings"));
            ui.separator();

            egui::Grid::new("general_settings_grid")
                .striped(true)
                .spacing([20.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Gravity:");
                    ui.add(egui::Slider::new(&mut self.dp.gravity, 0.1..=20.0).fixed_decimals(2));
                    ui.end_row();

                    ui.label("Damping:");
                    ui.add(
                        egui::Slider::new(&mut self.dp.damping, 0.0..=1.0)
                            .fixed_decimals(3)
                            .step_by(0.01),
                    );
                    ui.end_row();

                    ui.label("Time step:");
                    ui.add(egui::Slider::new(&mut self.time_step, 0.01..=1.0).fixed_decimals(2));
                    ui.end_row();
                });
            ui.separator();

            ui.heading("First pendulum");
            egui::Grid::new("first_pendulum_grid")
                .striped(true)
                .spacing([20.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Mass:");
                    ui.add(
                        egui::Slider::new(&mut self.dp.pendula.0.mass, 5.0..=70.0)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Arm length:");
                    ui.add(
                        egui::Slider::new(&mut self.dp.pendula.0.arm_length, 10.0..=300.0)
                            .fixed_decimals(2),
                    );
                });
            ui.separator();

            ui.heading("Second pendulum");
            egui::Grid::new("second_pendulum_grid")
                .striped(true)
                .spacing([20.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Mass:");
                    ui.add(
                        egui::Slider::new(&mut self.dp.pendula.1.mass, 5.0..=70.0)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Arm length:");
                    ui.add(
                        egui::Slider::new(&mut self.dp.pendula.1.arm_length, 10.0..=300.0)
                            .fixed_decimals(2),
                    );
                });

            ui.separator();
            ui.collapsing(egui::RichText::new("Shortcuts").heading(), |ui| {
                egui::Grid::new("shortcuts_grid")
                    .striped(true)
                    .spacing([10.0, 5.0])
                    .show(ui, |ui| {
                        ui.label("Start / Stop:");
                        ui.label("Space");
                        ui.end_row();

                        ui.label("Reset:");
                        ui.label("Ctrl+Alt+R");
                        ui.end_row();

                        ui.label("Zoom in:");
                        ui.label("Ctrl++");
                        ui.end_row();

                        ui.label("Zoom in:");
                        ui.label("Ctrl+-");
                        ui.end_row();
                    });
            });
        });
    }

    fn canvas(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) = ui.allocate_painter(
                ui.available_size_before_wrap(),
                egui::Sense {
                    click: false,
                    drag: false,
                    focusable: false,
                },
            );

            self.canvas_size = response.rect.size();
            self.canvas_transform = emath::RectTransform::from_to(
                egui::Rect::from_min_size(egui::Pos2::ZERO, response.rect.size()),
                response.rect,
            );

            painter.add(egui::Shape::line(
                self.position_history
                    .iter()
                    .map(|(_, p)| self.canvas_transform * p.1)
                    .collect(),
                egui::Stroke {
                    width: 1.0,
                    color: egui::Color32::GRAY,
                },
            ));

            self.paint_pendulum(&painter, &self.dp.pendula.0);
            self.paint_pendulum(&painter, &self.dp.pendula.1);
        });
    }

    fn paint_pendulum(&self, painter: &egui::Painter, pendulum: &Pendulum) {
        painter.line_segment(
            [
                self.canvas_transform * pendulum.pivot,
                self.canvas_transform * pendulum.position(),
            ],
            egui::Stroke {
                width: 3.0,
                color: egui::Color32::WHITE,
            },
        );

        painter.circle_filled(
            self.canvas_transform * pendulum.position(),
            pendulum.mass * MASS_COEFFICIENT,
            egui::Color32::WHITE,
        );
    }

    fn plots(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("graph_tabs").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.current_plot,
                        Plot::Angle,
                        egui::RichText::new("Angles").heading(),
                    );
                    ui.selectable_value(
                        &mut self.current_plot,
                        Plot::Velocity,
                        egui::RichText::new("Velocities").heading(),
                    );
                    ui.selectable_value(
                        &mut self.current_plot,
                        Plot::Acceleration,
                        egui::RichText::new("Accelerations").heading(),
                    );
                    ui.selectable_value(
                        &mut self.current_plot,
                        Plot::Position,
                        egui::RichText::new("Positions").heading(),
                    );
                });
            });

            egui::CentralPanel::default().show_inside(ui, |ui| match self.current_plot {
                Plot::Angle => self.angle_plot(ui),
                Plot::Velocity => self.velocity_plot(ui),
                Plot::Acceleration => self.acceleration_plot(ui),
                Plot::Position => self.position_plot(ui),
            });
        });
    }

    fn angle_plot(&self, ui: &mut egui::Ui) {
        let theta1: egui_plot::PlotPoints = self
            .angle_history
            .iter()
            .map(|(time, (theta, _))| [time, theta as f64])
            .collect();

        let theta2: egui_plot::PlotPoints = self
            .angle_history
            .iter()
            .map(|(time, (_, theta))| [time, theta as f64])
            .collect();

        egui_plot::Plot::new("angle")
            .center_y_axis(true)
            .allow_zoom(true)
            .allow_scroll(false)
            .allow_drag(false)
            .x_grid_spacer(egui_plot::uniform_grid_spacer(|_| [10.0, 5.0, 1.0]))
            .y_grid_spacer(egui_plot::uniform_grid_spacer(|_| [TAU, PI, PI / 2.0]))
            .y_axis_formatter(|value, _, _| {
                if value == 0.0 {
                    "0".to_string()
                } else {
                    format!("{}π", value / PI)
                }
            })
            .x_axis_label("Time")
            .y_axis_label("Angle")
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(egui_plot::Line::new(theta1).name("First pendulum θ"));
                plot_ui.line(egui_plot::Line::new(theta2).name("Second pendulum θ"));
            });
    }

    fn velocity_plot(&self, ui: &mut egui::Ui) {
        let v1: egui_plot::PlotPoints = self
            .velocity_history
            .iter()
            .map(|(time, (v, _))| [time, v as f64])
            .collect();

        let v2: egui_plot::PlotPoints = self
            .velocity_history
            .iter()
            .map(|(time, (_, v))| [time, v as f64])
            .collect();

        egui_plot::Plot::new("velocity")
            .center_y_axis(true)
            .allow_zoom(true)
            .allow_scroll(false)
            .allow_drag(false)
            .x_axis_label("Time")
            .y_axis_label("Velocity")
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(egui_plot::Line::new(v1).name("First pendulum velocity"));
                plot_ui.line(egui_plot::Line::new(v2).name("Second pendulum velocity"));
            });
    }

    fn acceleration_plot(&self, ui: &mut egui::Ui) {
        let a1: egui_plot::PlotPoints = self
            .acceleration_history
            .iter()
            .map(|(time, (a, _))| [time, a as f64])
            .collect();

        let a2: egui_plot::PlotPoints = self
            .acceleration_history
            .iter()
            .map(|(time, (_, a))| [time, a as f64])
            .collect();

        egui_plot::Plot::new("acceleration")
            .center_y_axis(true)
            .allow_zoom(true)
            .allow_scroll(false)
            .allow_drag(false)
            .x_axis_label("Time")
            .y_axis_label("Acceleration")
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(egui_plot::Line::new(a1).name("First pendulum acceleration"));
                plot_ui.line(egui_plot::Line::new(a2).name("Second pendulum acceleration"));
            });
    }

    fn position_plot(&self, ui: &mut egui::Ui) {
        let p1: egui_plot::PlotPoints = self
            .position_history
            .iter()
            .map(|(_, (pos, _))| self.canvas_transform * pos - self.dp.pendula.0.pivot)
            .map(|pos| [pos.x as f64, -pos.y as f64])
            .collect();

        let p2: egui_plot::PlotPoints = self
            .position_history
            .iter()
            .map(|(_, (_, pos))| self.canvas_transform * pos - self.dp.pendula.0.pivot)
            .map(|pos| [pos.x as f64, -pos.y as f64])
            .collect();

        egui_plot::Plot::new("velocities")
            // .view_aspect(1.0)
            // .data_aspect(1.0)
            .legend(egui_plot::Legend::default())
            .show(ui, |plot_ui| {
                plot_ui.line(egui_plot::Line::new(p1).name("First pendulum position"));
                plot_ui.line(egui_plot::Line::new(p2).name("Second pendulum position"));
            });
    }

    fn move_pendula(&mut self, ctx: &egui::Context) {
        let pointer_position = ctx.input(|i| {
            if i.pointer.primary_released() {
                self.moving.0 = false;
                self.moving.1 = false;
            }

            if let Some(pointer_position) = i.pointer.latest_pos().map(|p| p.to_vec2()) {
                if i.pointer.primary_pressed() {
                    if (self.canvas_transform * self.dp.pendula.0.position() - pointer_position)
                        .to_vec2()
                        .length_sq()
                        < (self.dp.pendula.0.mass * MASS_COEFFICIENT).powi(2)
                    {
                        self.moving.0 = true;
                    }

                    if (self.canvas_transform * self.dp.pendula.1.position() - pointer_position)
                        .to_vec2()
                        .length_sq()
                        < (self.dp.pendula.1.mass * MASS_COEFFICIENT).powi(2)
                    {
                        self.moving.1 = true;
                    }
                }

                pointer_position
            } else {
                egui::Vec2::ZERO
            }
        });

        if self.moving.0 {
            self.dp.pendula.0.angle = (pointer_position
                - (self.canvas_transform * self.dp.pendula.0.pivot).to_vec2())
            .yx()
            .angle();
            self.dp.pendula.0.acceleration = 0.0;
            self.dp.pendula.0.velocity = 0.0;

            self.dp.pendula.1.pivot = self.canvas_transform * self.dp.pendula.0.position();
            self.dp.pendula.1.acceleration = 0.0;
            self.dp.pendula.1.velocity = 0.0;
            self.position_history.clear();
        }

        if self.moving.1 {
            self.dp.pendula.0.acceleration = 0.0;
            self.dp.pendula.0.velocity = 0.0;

            self.dp.pendula.1.angle = (pointer_position
                - (self.canvas_transform * self.dp.pendula.1.pivot).to_vec2())
            .yx()
            .angle();
            self.dp.pendula.1.acceleration = 0.0;
            self.dp.pendula.1.velocity = 0.0;
            self.position_history.clear();
        }
    }

    fn record_history(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        let now = self.epoch.elapsed().as_millis() as f64 / 1000.0;

        #[cfg(target_arch = "wasm32")]
        let now = eframe::web::now_sec();

        self.position_history.add(
            now,
            (self.dp.pendula.0.position(), self.dp.pendula.1.position()),
        );

        self.angle_history.add(
            now,
            (
                self.dp.pendula.0.angle.sin().asin(),
                self.dp.pendula.1.angle.sin().asin(),
            ),
        );

        self.velocity_history.add(
            now,
            (self.dp.pendula.0.velocity, self.dp.pendula.1.velocity),
        );

        self.acceleration_history.add(
            now,
            (
                self.dp.pendula.0.acceleration,
                self.dp.pendula.1.acceleration,
            ),
        );
    }

    #[inline]
    fn moving(&self) -> bool {
        self.moving.0 || self.moving.1
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn reset(&mut self) {
        self.dp = DoublePendulum::default();
        self.position_history.clear();
        self.angle_history.clear();
        self.velocity_history.clear();
        self.acceleration_history.clear();
        self.epoch = std::time::Instant::now();
    }

    #[cfg(target_arch = "wasm32")]
    fn reset(&mut self) {
        self.dp = DoublePendulum::default();
        self.position_history.clear();
        self.angle_history.clear();
        self.velocity_history.clear();
        self.acceleration_history.clear();
    }
}
