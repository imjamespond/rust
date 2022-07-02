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
    pub page_w: f64,
    pub page_h: f64,
    pub width: f64,
    pub height: f64,
    pub margin_x: f64,
    pub margin_y: f64,
    pub page_size: f64,
    pub cols: u32,
}

pub fn read_config() -> Config {
    let data = fs::read_to_string("./config.json").expect("Unable to read file");
    let config: Config/*  serde_json::Value */ =
        serde_json::from_str(&data).expect("JSON does not have correct format.");
    config
}

pub fn save_config(config: &Config) {
    let serialized = serde_json::to_string(config).unwrap();
    fs::write("./test.config.json", serialized).ok();
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
        use super::Config;
        use super::Pdf;
        use super::QRCode;

        let cfg = Config {
            qrcode: QRCode {
                x: 1.,
                y: 2.,
                width: 0.1,
                height: 0.2,
            },
            pdf: Pdf {
                page_h: 0.,
                page_w: 0.,
                page_size: 0.,
                margin_x: 0.,
                margin_y: 0.,
                width: 0.,
                height: 0.,
                cols: 0,
            },
        };

        // println!("{:?}", cfg);
        super::save_config(&cfg);
    }
}
