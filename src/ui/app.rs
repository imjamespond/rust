use super::font::setup_custom_fonts;
use crate::{
    config::{read_config, save_config, Config},
    pdf, qrcode,
};
use eframe::egui;

pub struct MyApp {
    // text: String,
    config: Config,
    pdf: pdf::Pdf,
    qrcode: qrcode::QRCode,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let config = read_config();
        let mut pdf = pdf::Pdf::default();
        let qrcode = qrcode::QRCode::default();
        pdf.set_pdf_tool_root(); // 改变当前路径，所有取相对路径在之前完成
        Self {
            // text: "Edit this text field if you want".to_owned(),
            config,
            pdf,
            qrcode,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Config { qrcode, pdf: _ } = &mut self.config;
            ui.label("qrcode 配置");
            // ui.text_edit_multiline(&mut self.text);
            ui.horizontal(|ui| {
                ui.label("宽: ");
                ui.add(egui::DragValue::new(&mut qrcode.width).speed(1.0));
            });

            ui.label("pdf 配置");

            if ui.button("保存").clicked() {
                save_config(&self.config, "./config.json");
            }

            ui.label("操作");

            ui.horizontal(|ui| {
                if ui.button("合成qrcode").clicked() {
                    println!("merge btn clicks");
                    self.qrcode.qrcodes();
                }
                if self.pdf.pdf_tool_dir {
                    if ui.button("生成pdf").clicked() {
                        self.pdf.exec();
                    }
                } else {
                    if ui.button("安装node pdf").clicked() {
                        self.pdf.npm_install();
                    }
                }
            });
        });
    }
}
