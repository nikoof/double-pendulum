use eframe::egui::{vec2, Vec2};

#[derive(Clone, Copy, PartialEq)]
pub struct Pendulum {
    pub pivot: Vec2,
    pub arm_length: f32,
    pub angle: f32,
    pub mass: f32,
}

impl Default for Pendulum {
    fn default() -> Self {
        Self::new([0.0, 0.0], 100.0, 0.0, 1.0)
    }
}

impl Pendulum {
    fn new(pivot: impl Into<Vec2>, arm_length: f32, angle: f32, mass: f32) -> Self {
        Self {
            pivot: pivot.into(),
            arm_length,
            angle,
            mass,
        }
    }

    #[inline]
    pub fn position(&self) -> Vec2 {
        self.pivot + self.arm_length * vec2(self.angle.sin(), self.angle.cos())
    }
}
