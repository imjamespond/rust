use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::util::get_abs_path;

// #[derive(Debug)] // have rust extend the debug trait.
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Config {
    pub qrcode: QRCode,
    pub pdf: Pdf,
}

impl Default for Config {
    fn default() -> Self {
        Self { qrcode: Default::default(), pdf: Default::default() }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct QRCode {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
#[derive(Serialize, Deserialize, Clone, Copy, Default)]
pub struct Pdf {
    pub padding_x: Option<f64>,
    pub padding_y: Option<f64>,
    pub page_w: f64,
    pub page_h: f64,
    pub page_num_x: Option<f64>,
    pub page_num_y: Option<f64>,
    pub page_size: u32,
    pub width: f64,
    pub height: f64,
    pub margin_x: f64,
    pub margin_y: f64,
    pub cols: u32,
    pub size: u32, // qrcode数量
}

pub fn read_config() -> (Config, PathBuf) {
    let config_file = get_abs_path(&["config.json"]);
    let data = fs::read_to_string(config_file.to_str().unwrap()).expect("Unable to read file");
    let config: Config/*  serde_json::Value */ =
        serde_json::from_str(&data).expect("JSON does not have correct format.");
    (config, config_file)
}

pub fn save_config(config: &Config, config_file: &PathBuf) {
    let serialized = serde_json::to_string_pretty(config).unwrap();
    fs::write(config_file.to_str().unwrap(), serialized).ok();
}

mod tests {

    #[test]
    fn test_fromjson() {
        use super::read_config;
        let (config, _config_file) = read_config();
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn test_tojson() {
        use crate::util::get_abs_path;
        
        let config = super::Config {
            qrcode: super::QRCode{..Default::default()},
            pdf: super::Pdf{..Default::default()},
        };

        // println!("{:?}", cfg);
        let config_file = get_abs_path(&["test.config.json"]);
        super::save_config(&config, &config_file);
    }
}
