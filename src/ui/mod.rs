pub mod font;
pub mod app;

use eframe;

pub fn run(){
  let options = eframe::NativeOptions::default();
  eframe::run_native(
      "qrcode generate",
      options,
      Box::new(|cc| {
        println!("run_native app_creator");
        Box::new(app::MyApp::new(cc))
      }),
  );
}