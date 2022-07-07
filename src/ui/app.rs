use std::path::PathBuf;

use super::font::setup_custom_fonts;
use crate::{
    config::{read_config, save_config, Config},
    pdf, qrcode,
};
use eframe::egui;

pub struct MyApp {
    config_file: PathBuf,
    config: Config,
    pdf: pdf::Pdf,
    qrcode: qrcode::QRCode,

    padding_x: f64,
    padding_y: f64,
    page_num_x: f64,
    page_num_y: f64,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let (config, config_file) = read_config();
        let mut pdf = pdf::Pdf::default();
        let qrcode = qrcode::QRCode::default();
        pdf.set_pdf_tool_root(); // 改变当前路径，所有取相对路径在之前完成

        let padding_x = match config.pdf.padding_x {
            Some(val) => val,
            _ => 0.0,
        };
        let padding_y = match config.pdf.padding_y {
            Some(val) => val,
            _ => 0.0,
        };
        let page_num_x = match config.pdf.page_num_x {
            Some(val) => val,
            _ => 0.0,
        };
        let page_num_y = match config.pdf.page_num_y {
            Some(val) => val,
            _ => 0.0,
        };

        Self {
            config_file,
            config,
            pdf,
            qrcode,
            padding_x,
            padding_y,
            page_num_x,
            page_num_y,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Config {
                qrcode,
                pdf: pdf_conifg,
            } = &mut self.config;
            pdf_conifg.padding_x = Some(self.padding_x);
            pdf_conifg.padding_y = Some(self.padding_y);
            pdf_conifg.page_num_x = Some(self.page_num_x);
            pdf_conifg.page_num_y = Some(self.page_num_y);
            ui.label("qrcode 配置:");
            ui.horizontal(|ui| {
                ui.label("宽: ");
                ui.add(egui::DragValue::new(&mut qrcode.width).speed(1.0));
                ui.label("高: ");
                ui.add(egui::DragValue::new(&mut qrcode.height).speed(1.0));
            });
            ui.label("qrcode 位置");
            ui.horizontal(|ui| {
                ui.label("x : ");
                ui.add(egui::DragValue::new(&mut qrcode.x).speed(1.0));
                ui.label("y : ");
                ui.add(egui::DragValue::new(&mut qrcode.y).speed(1.0));
            });
            ui.separator();

            ui.label("PDF 配置:");
            ui.horizontal(|ui| {
                ui.label("padding x : ");
                ui.add(egui::DragValue::new(&mut self.padding_x).speed(1.0));
                ui.label("padding y : ");
                ui.add(egui::DragValue::new(&mut self.padding_y).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("贺卡宽 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.width).speed(1.0));
                ui.label("贺卡高 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.height).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("间距x : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.margin_x).speed(1.0));
                ui.label("间距y : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.margin_y).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("PDF宽 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.page_w).speed(1.0));
                ui.label("PDF高 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.page_h).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("页码x : ");
                ui.add(egui::DragValue::new(&mut self.page_num_x).speed(1.0));
                ui.label("页码y : ");
                ui.add(egui::DragValue::new(&mut self.page_num_y).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("列数 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.cols).speed(1.0));
                ui.label("PDF每页数量 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.page_size).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("每个PDF数量 : ");
                ui.add(egui::DragValue::new(&mut pdf_conifg.size).speed(1.0));
            });
            if ui.button("保存配置").clicked() {
                save_config(&self.config, &self.config_file);
            }
            ui.separator();

            ui.label("操作:");

            ui.horizontal(|ui| {
                if ui.button("合成qrcode").clicked() {
                    println!("merge btn clicks");
                    self.qrcode.qrcodes(&self.config);
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
