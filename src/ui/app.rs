use super::font::setup_custom_fonts;
use crate::{
    config::{read_config, save_config, Config},
    pdf::{ Pdf, npm_install},
};
use eframe::egui;

pub struct MyApp {
    // text: String,
    config: Config,
    pdf: Pdf,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            // text: "Edit this text field if you want".to_owned(),
            config: read_config(),
            pdf: Pdf::default()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Config { qrcode, pdf: _ } = &mut self.config;
            ui.heading("qrcode 配置");
            // ui.text_edit_multiline(&mut self.text);
            ui.horizontal(|ui| {
                ui.label("宽: ");
                ui.add(egui::DragValue::new(&mut qrcode.width).speed(1.0));
            });

            ui.heading("pdf 配置");

            if ui.button("保存").clicked() {
                save_config(&self.config, "./config.json");
            }

            ui.horizontal(|ui| {
                if ui.button("生成pdf").clicked() {
                    self.pdf.exec();
                }
                if ui.button("安装node pdf").clicked() {
                    npm_install();
                }
            });
        });
    }
}
