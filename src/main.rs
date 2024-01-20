mod app;
mod pendulum;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app::App::default())),
    )
}
