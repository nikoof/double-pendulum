use eframe::egui;

mod pendulum;

use pendulum::Pendulum;

const MASS_COEFFICIENT: f32 = 10.0;

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
            })
        }),
    )
}

struct App {
    moving_one: bool,
    moving_two: bool,

    pendulum_one: Pendulum,
    pendulum_two: Pendulum,
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
    fn input(&mut self, ctx: &egui::Context) {
        let pointer_position = ctx.input(|i| {
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

                if i.pointer.primary_released() {
                    self.moving_one = false;
                    self.moving_two = false;
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

    fn ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Controls").show(ctx, |ui| {
            ui.label("Test");
            ui.label("Test");
            ui.label("Test");
            ui.label("Test");
            ui.label("Test");
            ui.label("Test");
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
}
