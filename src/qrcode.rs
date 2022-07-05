use image::{DynamicImage, GenericImage};
use std::fs;
use std::path::Path;
use std::time::{Instant};

static IMG_TPL_PATH: &str = "./qrcode/template.jpg";

pub fn qrcodes() {
    let timer = Instant::now();

    let tpl = self::open(IMG_TPL_PATH);

    fs::create_dir_all("./qrcode/output").ok();
    let input_dir = Path::new("./qrcode/input");
    let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
    for entry in input_dir.read_dir().expect("read_dir call failed") {
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

                    self::draw(&tpl, &qrcode, file_name);
                }
            }
        }
    }

    println!("generate qrcode in {}", timer.elapsed().as_millis());
}

fn draw(tpl: &DynamicImage, qrcode: &DynamicImage, qrcode_name: &str) {
    let mut tpl_new = tpl.clone();
    tpl_new.copy_from(qrcode, 200, 100).unwrap();
    tpl_new
        .save(format!("./qrcode/output/{}", qrcode_name))
        .unwrap();
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
        use super::qrcodes;

        qrcodes();
    }

    #[test]
    fn test_opentpl() {
        use super::open;
        use super::IMG_TPL_PATH;

        open(IMG_TPL_PATH);
    }
}
