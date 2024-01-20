use eframe::egui;
use pendulum::Pendulum;

mod pendulum;

const MASS_COEFFICIENT: f32 = 1.0;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Box::new(App {
                moving_one: false,
                moving_two: false,

                pendulum_one: Pendulum::default(),
                pendulum_two: Pendulum::default(),

                gravity: 9.81,
                damping: 0.001,
            })
        }),
    )
}

struct App {
    moving_one: bool,
    moving_two: bool,

    pendulum_one: Pendulum,
    pendulum_two: Pendulum,

    gravity: f32,
    damping: f32,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.pendulum_one.pivot = egui::vec2(ctx.screen_rect().size().x / 2.0, 0.0);
        self.pendulum_two.pivot = self.pendulum_one.position();

        self.input(ctx);

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
                    ui.label(egui::RichText::from("Gravity:"));
                    ui.add(egui::Slider::new(&mut self.gravity, 0.1..=100.0).fixed_decimals(2));
                    ui.end_row();

                    ui.label("Damping:");
                    ui.add(
                        egui::Slider::new(&mut self.damping, 0.0..=1.0)
                            .fixed_decimals(3)
                            .step_by(0.01),
                    );
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
                        egui::Slider::new(&mut self.pendulum_one.mass, 5.0..=70.0)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Arm length:");
                    ui.add(
                        egui::Slider::new(&mut self.pendulum_one.arm_length, 10.0..=300.0)
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
                        egui::Slider::new(&mut self.pendulum_two.mass, 5.0..=70.0)
                            .fixed_decimals(2),
                    );
                    ui.end_row();

                    ui.label("Arm length:");
                    ui.add(
                        egui::Slider::new(&mut self.pendulum_two.arm_length, 10.0..=300.0)
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

        self.paint_pendulum(&painter, &self.pendulum_one);
        self.paint_pendulum(&painter, &self.pendulum_two);
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
                    if (self.pendulum_one.position() - pointer_position).length_sq()
                        < (self.pendulum_one.mass * MASS_COEFFICIENT).powi(2)
                    {
                        self.moving_one = true;
                    }

                    if (self.pendulum_two.position() - pointer_position).length_sq()
                        < (self.pendulum_two.mass * MASS_COEFFICIENT).powi(2)
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
            self.pendulum_one.angle = (pointer_position - self.pendulum_one.pivot).yx().angle();
            self.pendulum_two.pivot = self.pendulum_one.position();
        }

        if self.moving_two {
            self.pendulum_two.angle = (pointer_position - self.pendulum_two.pivot).yx().angle();
        }
    }
}
