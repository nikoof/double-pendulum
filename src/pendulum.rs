use eframe::egui::{vec2, Pos2};

#[derive(Clone, Copy, PartialEq)]
pub struct Pendulum {
    pub pivot: Pos2,
    pub arm_length: f32,
    pub angle: f32,
    pub mass: f32,
    pub velocity: f32,
    pub acceleration: f32,
}

impl Default for Pendulum {
    fn default() -> Self {
        Self::new([0.0, 0.0], 100.0, 20.0, 0.0, 0.0, 0.0)
    }
}

impl Pendulum {
    pub fn new(
        pivot: impl Into<Pos2>,
        arm_length: f32,
        mass: f32,
        angle: f32,
        velocity: f32,
        acceleration: f32,
    ) -> Self {
        Self {
            pivot: pivot.into(),
            arm_length,
            angle,
            mass,
            velocity,
            acceleration,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.angle += self.velocity * delta_time;
        self.velocity += self.acceleration * delta_time;
    }

    #[inline]
    pub fn position(&self) -> Pos2 {
        self.pivot + self.arm_length * vec2(self.angle.sin(), self.angle.cos())
    }
}

pub struct DoublePendulum {
    pub pendula: (Pendulum, Pendulum),
    pub gravity: f32,
    pub damping: f32,
}

impl Default for DoublePendulum {
    fn default() -> Self {
        Self {
            pendula: (Pendulum::default(), Pendulum::default()),
            gravity: 1.0,
            damping: 0.005,
        }
    }
}

impl DoublePendulum {
    pub fn update(&mut self, delta_time: f32) {
        self.pendula.0.update(delta_time);
        self.pendula.1.update(delta_time);

        let g = self.gravity;

        let t1 = self.pendula.0.angle;
        let m1 = self.pendula.0.mass;
        let v1 = self.pendula.0.velocity;
        let l1 = self.pendula.0.arm_length;

        let t2 = self.pendula.1.angle;
        let m2 = self.pendula.1.mass;
        let v2 = self.pendula.1.velocity;
        let l2 = self.pendula.1.arm_length;

        self.pendula.0.acceleration = (-g * (2.0 * m1 + m2) * t1.sin()
            - m2 * g * (t1 - 2.0 * t2).sin()
            - 2.0 * (t1 - t2).sin() * m2 * (v2 * v2 * l2 + v1 * v1 * l1 * (t1 - t2).cos()))
            / (l1 * (2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos()));
        self.pendula.0.acceleration *= 1.0 - self.damping;

        self.pendula.1.acceleration = (2.0
            * (t1 - t2).sin()
            * (v1 * v1 * l1 * (m1 + m2)
                + g * (m1 + m2) * t1.cos()
                + v2 * v2 * l2 * m2 * (t1 - t2).cos()))
            / (l2 * (2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos()));
        self.pendula.1.acceleration *= 1.0 - self.damping;
    }
}
