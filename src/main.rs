#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod pendulum;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Double Pendulum",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(app::App::default())),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    #[cfg(debug_assertions)]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "double_pendulum_canvas",
                eframe::WebOptions::default(),
                Box::new(|_cc| Box::new(app::App::default())),
            )
            .await
            .expect("Failed to initialize eframe")
    });
}
