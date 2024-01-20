use crate::pendulum::{DoublePendulum, Pendulum};
use eframe::egui;

const MASS_COEFFICIENT: f32 = 1.0;

pub struct App {
    moving_one: bool,
    moving_two: bool,

    dp: DoublePendulum,

    time_step: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            moving_one: false,
            moving_two: false,

            dp: DoublePendulum::default(),

            time_step: 0.4,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.input(ctx);

        if !(self.moving_one || self.moving_two) {
            self.dp.update(self.time_step);
        }

        self.dp.pendula.0.pivot = egui::vec2(
            ctx.screen_rect().size().x / 2.0,
            ctx.screen_rect().size().y / 2.0,
        );
        self.dp.pendula.1.pivot = self.dp.pendula.0.position();

        self.canvas(ctx);
        self.ui(ctx);

        ctx.request_repaint();
    }
}

impl App {
    fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Controls").show(ctx, |ui| {
            egui::Grid::new("general_controls_grid")
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
        });
    }

    fn canvas(&mut self, ctx: &egui::Context) {
        let painter = ctx.layer_painter(egui::LayerId {
            order: egui::layers::Order::Background,
            id: "bg".into(),
        });

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

    fn input(&mut self, ctx: &egui::Context) {
        let pointer_position = ctx.input(|i| {
            if i.pointer.primary_released() {
                self.moving_one = false;
                self.moving_two = false;
            }

            if let Some(pointer_position) = i.pointer.latest_pos().map(|p| p.to_vec2()) {
                if i.pointer.primary_pressed() {
                    if (self.dp.pendula.0.position() - pointer_position).length_sq()
                        < (self.dp.pendula.0.mass * MASS_COEFFICIENT).powi(2)
                    {
                        self.moving_one = true;
                    }

                    if (self.dp.pendula.1.position() - pointer_position).length_sq()
                        < (self.dp.pendula.1.mass * MASS_COEFFICIENT).powi(2)
                    {
                        self.moving_two = true;
                    }
                }

                pointer_position
            } else {
                egui::Vec2::ZERO
            }
        });

        if self.moving_one {
            self.dp.pendula.0.angle = (pointer_position - self.dp.pendula.0.pivot).yx().angle();
            self.dp.pendula.1.pivot = self.dp.pendula.0.position();
        }

        if self.moving_two {
            self.dp.pendula.1.angle = (pointer_position - self.dp.pendula.1.pivot).yx().angle();
        }
    }
}
