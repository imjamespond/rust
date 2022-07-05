use serde::{Deserialize, Serialize};
use std::fs;

// #[derive(Debug)] // have rust extend the debug trait.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub qrcode: QRCode,
    pub pdf: Pdf,
}
#[derive(Serialize, Deserialize)]
pub struct QRCode {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}
#[derive(Serialize, Deserialize)]
pub struct Pdf {
    pub padding_x: Option<f64>,
    pub padding_y: Option<f64>,
    pub page_w: f64,
    pub page_h: f64,
    pub page_num_x: Option<f64>,
    pub page_num_y: Option<f64>,
    pub page_size: f64,
    pub width: f64,
    pub height: f64,
    pub margin_x: f64,
    pub margin_y: f64,
    pub cols: u32,
}

impl Default for QRCode {
    fn default() -> Self {
        QRCode {
            x: 0.,
            y: 0.,
            width: 0.,
            height: 0.,
        }
    }
}

impl Default for Pdf {
    fn default() -> Self {
        Pdf {
            padding_x: Some(0.),
            padding_y: Some(0.),
            width: 0.,
            height: 0.,
            page_h: 0.,
            page_w: 0.,
            page_num_x: Some(0.),
            page_num_y: Some(0.),
            page_size: 0.,
            margin_x: 0.,
            margin_y: 0.,
            cols: 0,
        }
    }
}

pub fn read_config() -> Config {
    let data = fs::read_to_string("./config.json").expect("Unable to read file");
    let config: Config/*  serde_json::Value */ =
        serde_json::from_str(&data).expect("JSON does not have correct format.");
    config
}

pub fn save_config(config: &Config, config_file: &str) {
    let serialized = serde_json::to_string(config).unwrap();
    fs::write(config_file, serialized).ok();
}

mod tests {

    #[test]
    fn test_fromjson() {
        use super::read_config;
        let config = read_config();
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn test_tojson() {
        let config = super::Config {
            qrcode: super::QRCode::default(),
            pdf: super::Pdf::default(),
        };

        // println!("{:?}", cfg);
        super::save_config(&config, "./test.config.json");
    }
}
