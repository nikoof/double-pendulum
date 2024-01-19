use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| {
            Box::new(App {
                theta1: 0.0,
                theta2: 0.0,
                moving1: false,
                moving2: false,
            })
        }),
    )
}

struct App {
    theta1: f32,
    theta2: f32,
    moving1: bool,
    moving2: bool,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_size = ctx.screen_rect().size();
        let origin = egui::vec2(screen_size.x / 2.0, 0.0);

        ctx.input(|i| {
            let p1 = Self::angle_to_pos(origin, self.theta1, 100.0);
            let p2 = Self::angle_to_pos(p1, self.theta2, 100.0);
            if let Some(pp) = i.pointer.latest_pos().map(|p| p.to_vec2()) {
                if i.pointer.primary_pressed() {
                    if (p1 - pp).length_sq() < 100.0 {
                        self.moving1 = true;
                    }

                    if dbg!(p2 - pp).length_sq() < 100.0 {
                        self.moving2 = true;
                    }
                }

                if i.pointer.primary_released() {
                    self.moving1 = false;
                    self.moving2 = false;
                }

                if self.moving1 {
                    self.theta1 = Self::pos_to_angle(origin, pp);
                }

                if self.moving2 {
                    self.theta2 = Self::pos_to_angle(p1, pp);
                }
            }
        });

        self.draw_pendulum(ctx);
        self.ui(ctx);

        ctx.request_repaint();
    }
}

impl App {
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

    fn draw_pendulum(&mut self, ctx: &egui::Context) {
        let layer = egui::LayerId {
            order: egui::layers::Order::Background,
            id: "bg".into(),
        };

        let painter = ctx.layer_painter(layer);

        let screen_size = ctx.screen_rect().size();
        let origin = egui::vec2(screen_size.x / 2.0, 0.0);

        let p1 = Self::angle_to_pos(origin, self.theta1, 100.0);
        let p2 = Self::angle_to_pos(p1, self.theta2, 100.0);

        // self.theta1 += 0.01;
        // self.theta2 += 0.03;

        painter.line_segment(
            [origin.to_pos2(), p1.to_pos2()],
            egui::Stroke {
                width: 3.0,
                color: egui::Color32::WHITE,
            },
        );
        painter.line_segment(
            [p1.to_pos2(), p2.to_pos2()],
            egui::Stroke {
                width: 3.0,
                color: egui::Color32::WHITE,
            },
        );

        painter.circle_filled(p1.to_pos2(), 10.0, egui::Color32::WHITE);
        painter.circle_filled(p2.to_pos2(), 10.0, egui::Color32::WHITE);
    }

    fn angle_to_pos(origin: egui::Vec2, theta: f32, arm_length: f32) -> egui::Vec2 {
        origin + arm_length * egui::vec2(theta.sin(), theta.cos())
    }

    fn pos_to_angle(origin: egui::Vec2, pos: egui::Vec2) -> f32 {
        (pos - origin).yx().angle()
    }
}
