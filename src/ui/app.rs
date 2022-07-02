use eframe::egui;

use super::font::setup_custom_fonts;

pub struct MyApp {
  text: String,
}

impl MyApp {
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
      setup_custom_fonts(&cc.egui_ctx);
      Self {
          text: "Edit this text field if you want".to_owned(),
      }
  }
}

impl eframe::App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      egui::CentralPanel::default().show(ctx, |ui| {
          ui.heading("egui using custom fonts");
          ui.text_edit_multiline(&mut self.text);
      });
  }
}