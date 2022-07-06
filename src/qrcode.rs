use image::{DynamicImage, GenericImage};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use crate::pdf;

static IMG_TPL_PATH: [&str; 2] = ["qrcode","template.jpg"];

pub struct QRCode {
    output_dir: PathBuf,
    input_dir: PathBuf,
    img_tpl_path: PathBuf
}

impl Default for QRCode {
    fn default() -> Self {
        let output_dir = pdf::get_abs_path(&["qrcode","output"]);
        let input_dir = pdf::get_abs_path(&["qrcode","input"]);
        Self{
            output_dir,input_dir, img_tpl_path: pdf::get_abs_path(&IMG_TPL_PATH)
        }
    }
}

impl QRCode {
    pub fn qrcodes(&self) {
        let timer = Instant::now(); 
        let tpl = self::open(self.img_tpl_path.as_path().to_str().unwrap());

        println!("merge qrcode start");
    
        fs::create_dir_all(self.output_dir.as_path()).ok(); 
        let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
        let qrcodes = self.input_dir.as_path().read_dir().expect("read_dir call failed");
        let mut i: u32 = 0;
        for entry in qrcodes {
            if let Ok(entry) = entry {
                let path_buf = entry.path();
                let img_path = &path_buf.as_path();
                if img_path.is_file() {
                    let extension = img_path.extension().unwrap().to_str().unwrap();
    
                    if exts.contains(&extension) {
                        //文件后缀判断
                        let file_name = img_path.file_name().unwrap().to_str().unwrap();
                        let qrcode = match image::open(img_path) {
                            Ok(image) => image,
                            _ => {
                                println!("{} 图片格式有误", file_name);
                                break;
                            }
                        };
    
                        self.draw(&tpl, &qrcode, file_name);
                        i = i + 1;
                        println!("merge done {}", i);
                    }
                }
            }
        }
    
        println!("generate qrcode in {}", timer.elapsed().as_millis());
    }
    
    fn draw(&self, tpl: &DynamicImage, qrcode: &DynamicImage, qrcode_name: &str) {
        let mut output_file = self.output_dir.clone();
        output_file.push(PathBuf::from(qrcode_name));
        let mut tpl_new = tpl.clone();
        tpl_new.copy_from(qrcode, 200, 100).unwrap();
        tpl_new
            .save(format!("{}", output_file.as_path().to_str().unwrap()))
            .unwrap();
    }
}




fn open(img_path: &str) -> DynamicImage {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(img_path).unwrap();
    // The dimensions method returns the images width and height.
    // println!("dimensions {:?}", img.dimensions());
    // The color method returns the image's `ColorType`.
    // println!("{:?}", img.color());
    img
}

mod tests {
    #[test]
    fn test_qrcodes() {
        use super::QRCode;
        let qrc = QRCode::default();
        qrc.qrcodes();
    }


}
