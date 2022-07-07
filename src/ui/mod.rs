pub mod app;
pub mod font;

use eframe;

pub fn run() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(eframe::egui::vec2(300.0, 500.0));
    eframe::run_native(
        "qrcode 生成 pdf 工具",
        options,
        Box::new(|cc| {
            println!("run_native app_creator");
            Box::new(app::MyApp::new(cc))
        }),
    );
}
