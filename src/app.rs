use crate::pendulum::{DoublePendulum, Pendulum};
use eframe::egui;

const MASS_COEFFICIENT: f32 = 1.0;

pub struct App {
    dp: DoublePendulum,
    lines: Vec<Vec<egui::Pos2>>,

    time_step: f32,

    running: bool,
    moving: (bool, bool),
}

impl Default for App {
    fn default() -> Self {
        Self {
            dp: DoublePendulum::default(),
            lines: vec![vec![]],

            time_step: 0.4,

            running: true,
            moving: (false, false),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.input(ctx);
        self.move_pendula(ctx);

        let screen_size = ctx.screen_rect().size();
        self.dp.pendula.0.pivot = egui::vec2(screen_size.x / 2.0, screen_size.y / 2.0);
        self.dp.pendula.1.pivot = self.dp.pendula.0.position();

        if self.running & !self.moving() {
            self.dp.update(self.time_step);

            let current_line = self.lines.last_mut().expect("Lines should never be empty");
            let pos = self.dp.pendula.1.position().to_pos2();
            if current_line.last() != Some(&pos) {
                current_line.push(pos);
            }
        }

        self.canvas(ctx);
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

            if i.consume_key(egui::Modifiers::CTRL, egui::Key::R) {
                self.reset();
            }
        });
    }

    fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("\u{2699} Settings").show(ctx, |ui| {
            egui::Grid::new("general_settings_grid")
                .striped(true)
                .spacing([20.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Gravity:");
                    ui.add(egui::Slider::new(&mut self.dp.gravity, 0.1..=100.0).fixed_decimals(2));
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
                    .spacing([20.0, 5.0])
                    .show(ui, |ui| {
                        ui.label("Start / Stop:");
                        ui.label("Space");
                        ui.end_row();

                        ui.label("Reset:");
                        ui.label("Ctrl+R");
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
        let painter = ctx.layer_painter(egui::LayerId {
            order: egui::layers::Order::Background,
            id: "bg".into(),
        });

        painter.extend(
            self.lines
                .iter()
                .filter(|line| line.len() >= 2)
                .map(|line| {
                    egui::Shape::line(
                        line.clone(),
                        egui::Stroke {
                            width: 1.0,
                            color: egui::Color32::GRAY,
                        },
                    )
                }),
        );

        self.paint_pendulum(&painter, &self.dp.pendula.0);
        self.paint_pendulum(&painter, &self.dp.pendula.1);
    }

    fn paint_pendulum(&self, painter: &egui::Painter, pendulum: &Pendulum) {
        painter.line_segment(
            [pendulum.pivot.to_pos2(), pendulum.position().to_pos2()],
            egui::Stroke {
                width: 3.0,
                color: egui::Color32::WHITE,
            },
        );

        painter.circle_filled(
            pendulum.position().to_pos2(),
            pendulum.mass * MASS_COEFFICIENT,
            egui::Color32::WHITE,
        );
    }

    fn move_pendula(&mut self, ctx: &egui::Context) {
        let pointer_position = ctx.input(|i| {
            if i.pointer.primary_released() {
                self.moving.0 = false;
                self.moving.1 = false;
            }

            if let Some(pointer_position) = i.pointer.latest_pos().map(|p| p.to_vec2()) {
                if i.pointer.primary_pressed() {
                    if (self.dp.pendula.0.position() - pointer_position).length_sq()
                        < (self.dp.pendula.0.mass * MASS_COEFFICIENT).powi(2)
                    {
                        self.moving.0 = true;
                    }

                    if (self.dp.pendula.1.position() - pointer_position).length_sq()
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
            self.dp.pendula.0.angle = (pointer_position - self.dp.pendula.0.pivot).yx().angle();
            self.dp.pendula.0.acceleration = 0.0;
            self.dp.pendula.0.velocity = 0.0;

            self.dp.pendula.1.pivot = self.dp.pendula.0.position();
            self.dp.pendula.1.acceleration = 0.0;
            self.dp.pendula.1.velocity = 0.0;
            self.lines = vec![vec![]];
        }

        if self.moving.1 {
            self.dp.pendula.0.acceleration = 0.0;
            self.dp.pendula.0.velocity = 0.0;

            self.dp.pendula.1.angle = (pointer_position - self.dp.pendula.1.pivot).yx().angle();
            self.dp.pendula.1.acceleration = 0.0;
            self.dp.pendula.1.velocity = 0.0;
            self.lines = vec![vec![]];
        }
    }

    #[inline]
    fn moving(&self) -> bool {
        self.moving.0 || self.moving.1
    }

    fn reset(&mut self) {
        self.dp = DoublePendulum::default();
        self.lines = vec![vec![]];
    }
}
