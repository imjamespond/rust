use serde::{Deserialize, Serialize};

// #[derive(Debug)] // have rust extend the debug trait.
#[derive(Serialize, Deserialize)]
pub struct Config {
    qrcode: QRCode,
    pdf: Pdf,
}
#[derive(Serialize, Deserialize)]
pub struct QRCode {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
#[derive(Serialize, Deserialize)]
pub struct Pdf {
    page_w: f64,
    page_h: f64,
    width: f64,
    height: f64,
    margin_x: f64,
    margin_y: f64,
    page_size: f64,
    cols: u32,
}

mod tests {

    #[test]
    fn test_fromjson() {
        use std::fs;
        use super::Config;
        let data = fs::read_to_string("./config.json").expect("Unable to read file");
        let json: Config/*  serde_json::Value */ =
            serde_json::from_str(&data).expect("JSON does not have correct format.");
        let serialized = serde_json::to_string(&json).unwrap();
        println!("serialized = {}", serialized);
    }

    #[test]
    fn test_tojson() {
        use super::Config;
        use super::Pdf;
        use super::QRCode;

        let pdf = Config {
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

        // println!("{:?}", pdf);

        let serialized = serde_json::to_string(&pdf).unwrap();
        println!("serialized = {}", serialized);
    }
}
