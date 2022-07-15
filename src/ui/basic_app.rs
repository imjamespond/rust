extern crate native_windows_gui as nwg;
use super::basic_app_ui::BasicAppUi;
use crate::config::{read_config, save_config, Config, Pdf, QRCode};
use crate::{pdf, qrcode};
use std::cell::RefCell;
use std::{error::Error, path::PathBuf, rc::Rc};

static PADDING: i32 = 10;
static MARGING: i32 = 20;
static LINE_H: i32 = 25;
static WIDTH: i32 = 400;
static HEIGHT: i32 = 520;

#[derive(Default)]
pub struct BasicApp {
    config: RefCell<Config> ,
    config_file: PathBuf,
    pdf: pdf::Pdf,
    qrcode: qrcode::QRCode,

    window: nwg::Window,
    qrcode_label: nwg::Label,
    qr_width_label: nwg::Label,
    qr_width_input: nwg::TextInput,
    qr_height_label: nwg::Label,
    qr_height_input: nwg::TextInput,
    qr_x_label: nwg::Label,
    qr_x_input: nwg::TextInput,
    qr_y_label: nwg::Label,
    qr_y_input: nwg::TextInput,

    pdf_label: nwg::Label,
    pdf_px_label: nwg::Label,
    pdf_px_input: nwg::TextInput,
    pdf_py_label: nwg::Label,
    pdf_py_input: nwg::TextInput,
    pdf_pagew_label: nwg::Label,
    pdf_pagew_input: nwg::TextInput,
    pdf_pageh_label: nwg::Label,
    pdf_pageh_input: nwg::TextInput,
    pdf_pagenumx_label: nwg::Label,
    pdf_pagenumx_input: nwg::TextInput,
    pdf_pagenumy_label: nwg::Label,
    pdf_pagenumy_input: nwg::TextInput,
    pdf_cardw_label: nwg::Label,
    pdf_cardw_input: nwg::TextInput,
    pdf_cardh_label: nwg::Label,
    pdf_cardh_input: nwg::TextInput,
    pdf_cardmx_label: nwg::Label,
    pdf_cardmx_input: nwg::TextInput,
    pdf_cardmy_label: nwg::Label,
    pdf_cardmy_input: nwg::TextInput,
    pdf_size_label: nwg::Label,
    pdf_size_input: nwg::TextInput,
    pdf_cols_label: nwg::Label,
    pdf_cols_input: nwg::TextInput,
    pdf_total_label: nwg::Label,
    pdf_total_input: nwg::TextInput,

    save_config_btn: nwg::Button,
    generate_card_btn: nwg::Button,
    generate_pdf_btn: nwg::Button,
    npm_install_btn: nwg::Button,
}

impl BasicApp {
    fn save_config(&self) {
        let rs = (|| -> Result<(), Box<dyn Error>> {
            let pdf_px = self.pdf_px_input.text().parse::<f64>()?;
            let config = Config {
                qrcode: QRCode {
                    x: self.qr_x_input.text().parse()?,
                    y: self.qr_y_input.text().parse()?,
                    width: self.qr_width_input.text().parse()?,
                    height: self.qr_height_input.text().parse()?,
                },
                pdf: Pdf {
                    padding_x: Some(pdf_px),
                    padding_y: Some(self.pdf_py_input.text().parse()?),
                    page_w: self.pdf_pagew_input.text().parse()?,
                    page_h: self.pdf_pageh_input.text().parse()?,
                    page_num_x: Some(self.pdf_pagenumx_input.text().parse()?),
                    page_num_y: Some(self.pdf_pagenumy_input.text().parse()?),
                    page_size: self.pdf_size_input.text().parse()?,
                    width: self.pdf_cardw_input.text().parse()?,
                    height: self.pdf_cardh_input.text().parse()?,
                    margin_x: self.pdf_cardmx_input.text().parse()?,
                    margin_y: self.pdf_cardmy_input.text().parse()?,
                    cols: self.pdf_cols_input.text().parse()?,
                    size: self.pdf_total_input.text().parse()?,
                },
            };
            save_config(&config, &self.config_file);
            self.config.replace(config);
            Ok(())
        })();
        match rs {
            Ok(_) => {
                nwg::modal_info_message(&self.window, "", &format!("保存配置成功！"));
            }
            Err(_err) => {
                nwg::modal_info_message(&self.window, "", &format!("数据有误！"));
            }
        }
    }

    fn generate_card(&self) {
        self.qrcode.qrcodes(&self.config.borrow());
    }

    fn generate_pdf(&self) {
        self.pdf.exec();
    }

    fn install_pdf_tool(&self) {
        self.pdf.npm_install();
        nwg::modal_info_message(&self.window, "", &format!("安装完毕，请重新启动程序！"));
        nwg::stop_thread_dispatch();
    }

    fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn add_label(
    wnd: &nwg::Window,
    label: &mut nwg::Label,
    pos: (i32, i32),
    width: i32,
    text: &str,
) -> Result<(), nwg::NwgError> {
    nwg::Label::builder()
        .size((width, LINE_H))
        .position(pos)
        .text(text)
        .parent(wnd)
        .build(label)?;
    return Ok(());
}

fn add_input(
    wnd: &nwg::Window,
    input: &mut nwg::TextInput,
    pos: (i32, i32),
    width: i32,
    text: &str,
) -> Result<(), nwg::NwgError> {
    return add_input_focus(wnd, input, pos, width, text, false);
}

fn add_input_focus(
    wnd: &nwg::Window,
    input: &mut nwg::TextInput,
    pos: (i32, i32),
    width: i32,
    text: &str,
    focus: bool,
) -> Result<(), nwg::NwgError> {
    nwg::TextInput::builder()
        .size((width, LINE_H))
        .position(pos)
        .text(text)
        .parent(wnd)
        .focus(focus)
        .build(input)?;
    return Ok(());
}

impl nwg::NativeUi<BasicAppUi> for BasicApp {
    fn build_ui(mut data: BasicApp) -> Result<BasicAppUi, nwg::NwgError> {
        use nwg::Event as E;

        let (config, config_file) = read_config();
        data.config_file = config_file;
        let Config { qrcode, pdf } = config;
        data.pdf.set_pdf_tool_root();

        // Controls
        nwg::Window::builder()
            .flags(nwg::WindowFlags::WINDOW | nwg::WindowFlags::VISIBLE)
            .size((WIDTH, HEIGHT))
            .position((300, 300))
            .title("PDF Tool")
            .build(&mut data.window)?;

        let x = PADDING;
        let y = PADDING;

        let label_w = 80;
        let input_w = 80;

        {
            add_label(
                &data.window,
                &mut &mut data.qrcode_label,
                (x, y),
                150,
                "QRCode配置：",
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            let label_w = 80;
            let input_w = 80;
            add_label(
                &data.window,
                &mut data.qr_width_label,
                (x, y),
                label_w,
                "宽：",
            )?;
            let x = x + label_w;
            add_input_focus(
                &data.window,
                &mut data.qr_width_input,
                (x, y),
                input_w,
                &qrcode.width.to_string(),
                true,
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.qr_height_label,
                (x, y),
                label_w,
                "高：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.qr_height_input,
                (x, y),
                input_w,
                &qrcode.height.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;
        {
            add_label(
                &data.window,
                &mut data.qr_x_label,
                (x, y),
                label_w,
                "坐标X：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.qr_x_input,
                (x, y),
                input_w,
                &qrcode.x.to_string(),
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.qr_y_label,
                (x, y),
                label_w,
                "坐标Y：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.qr_y_input,
                (x, y),
                input_w,
                &qrcode.y.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            add_label(
                &data.window,
                &mut &mut data.pdf_label,
                (x, y),
                150,
                "PDF配置：",
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            add_label(
                &data.window,
                &mut data.pdf_pagew_label,
                (x, y),
                label_w,
                "PDF宽：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_pagew_input,
                (x, y),
                input_w,
                &pdf.page_w.to_string(),
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.pdf_pageh_label,
                (x, y),
                label_w,
                "PDF高：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_pageh_input,
                (x, y),
                input_w,
                &pdf.page_h.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            add_label(
                &data.window,
                &mut data.pdf_cardw_label,
                (x, y),
                label_w,
                "贺卡宽：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_cardw_input,
                (x, y),
                input_w,
                &pdf.width.to_string(),
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.pdf_cardh_label,
                (x, y),
                label_w,
                "贺卡高：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_cardh_input,
                (x, y),
                input_w,
                &pdf.height.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            add_label(
                &data.window,
                &mut data.pdf_cardmx_label,
                (x, y),
                label_w,
                "贺卡间距X：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_cardmx_input,
                (x, y),
                input_w,
                &pdf.margin_x.to_string(),
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.pdf_cardmy_label,
                (x, y),
                label_w,
                "贺卡间距Y：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_cardmy_input,
                (x, y),
                input_w,
                &pdf.margin_y.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            add_label(
                &data.window,
                &mut data.pdf_px_label,
                (x, y),
                label_w,
                "页间距X：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_px_input,
                (x, y),
                input_w,
                &match pdf.padding_x {
                    Some(val) => val.to_string(),
                    None => "0".to_string(),
                },
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.pdf_py_label,
                (x, y),
                label_w,
                "页间距Y：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_py_input,
                (x, y),
                input_w,
                &match pdf.padding_y {
                    Some(val) => val.to_string(),
                    None => "0".to_string(),
                },
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            add_label(
                &data.window,
                &mut data.pdf_pagenumx_label,
                (x, y),
                label_w,
                "页码坐标X：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_pagenumx_input,
                (x, y),
                input_w,
                &match pdf.page_num_x {
                    Some(val) => val.to_string(),
                    None => "0".to_string(),
                },
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.pdf_pagenumy_label,
                (x, y),
                label_w,
                "页码坐标Y：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_pagenumy_input,
                (x, y),
                input_w,
                &match pdf.page_num_y {
                    Some(val) => val.to_string(),
                    None => "0".to_string(),
                },
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            let label_w = 120;
            let input_w = 50;
            add_label(
                &data.window,
                &mut data.pdf_cols_label,
                (x, y),
                label_w,
                "每页列数：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_cols_input,
                (x, y),
                input_w,
                &pdf.cols.to_string(),
            )?;

            let x = x + input_w + MARGING;
            add_label(
                &data.window,
                &mut data.pdf_size_label,
                (x, y),
                label_w,
                "每页PDF贺卡数：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_size_input,
                (x, y),
                input_w,
                &pdf.page_size.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            let label_w = 120;
            let input_w = 50;

            add_label(
                &data.window,
                &mut data.pdf_total_label,
                (x, y),
                label_w,
                "每个PDF贺卡数：",
            )?;
            let x = x + label_w;
            add_input(
                &data.window,
                &mut data.pdf_total_input,
                (x, y),
                input_w,
                &pdf.size.to_string(),
            )?;
        }

        let y = y + LINE_H + PADDING;

        {
            nwg::Button::builder()
                .size((WIDTH - 20, 35))
                .position((10, y))
                .text("保存配置")
                .parent(&data.window)
                .build(&mut data.save_config_btn)?;
        }

        let y = y + LINE_H + PADDING;

        {
            nwg::Button::builder()
                .size((WIDTH - 20, 35))
                .position((10, y))
                .text("合成qrcode")
                .parent(&data.window)
                .build(&mut data.generate_card_btn)?;
        }

        let y = y + LINE_H + PADDING;

        if data.pdf.pdf_tool_dir {
            nwg::Button::builder()
                .size((WIDTH - 20, 35))
                .position((10, y))
                .text("生成PDF")
                .parent(&data.window)
                .build(&mut data.generate_pdf_btn)?;
        }
        else {
            nwg::Button::builder()
                .size((WIDTH - 20, 35))
                .position((10, y))
                .text("安装PDF工具")
                .parent(&data.window)
                .build(&mut data.npm_install_btn)?;
        }

        // Wrap-up
        let ui = BasicAppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    E::OnButtonClick => {
                        if &handle == &ui.save_config_btn {
                            BasicApp::save_config(&ui);
                        } else if &handle == &ui.generate_pdf_btn {
                            BasicApp::generate_pdf(&ui);
                        } else if &handle == &ui.generate_card_btn {
                            BasicApp::generate_card(&ui);
                        } else if &handle == &ui.npm_install_btn {
                            BasicApp::install_pdf_tool(&mut &ui);
                        }
                    }
                    E::OnWindowClose => {
                        if &handle == &ui.window {
                            BasicApp::say_goodbye(&ui);
                        }
                    }
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.window.handle,
            handle_events,
        ));

        return Ok(ui);
    }
}
