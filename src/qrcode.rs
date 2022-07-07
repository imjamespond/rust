use image::{DynamicImage, GenericImage};
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Instant;

use crate::config::Config;
use crate::pdf;

static IMG_TPL_PATH: [&str; 2] = ["qrcode", "template.jpg"];

pub struct QRCode {
    output_dir: PathBuf,
    input_dir: PathBuf,
    img_tpl_path: PathBuf,
}

impl Default for QRCode {
    fn default() -> Self {
        let output_dir = pdf::get_abs_path(&["qrcode", "output"]);
        let input_dir = pdf::get_abs_path(&["qrcode", "input"]);
        Self {
            output_dir,
            input_dir,
            img_tpl_path: pdf::get_abs_path(&IMG_TPL_PATH),
        }
    }
}

impl QRCode {
    pub fn qrcodes(&self, config: &Config) {
        println!("merge qrcode open template");

        let timer = Instant::now();
        let tpl = self::open(self.img_tpl_path.as_path().to_str().unwrap());

        println!("merge qrcode start");

        fs::create_dir_all(self.output_dir.as_path()).ok();
        let exts: Vec<&str> = vec!["png", "jpg", "jpeg"];
        let qrcodes = self
            .input_dir
            .as_path()
            .read_dir()
            .expect("read_dir call failed");
        let mut count: u32 = 0;
        let mut qrcode_paths = Vec::new();
        for entry in qrcodes {
            if let Ok(entry) = entry {
                let path_buf = entry.path();
                let img_path = &path_buf.as_path();
                if img_path.is_file() {
                    let extension = img_path.extension().unwrap().to_str().unwrap();

                    if exts.contains(&extension) {
                        //文件后缀判断
                        qrcode_paths.push(path_buf);

                        count = count + 1;
                    }
                }
            }
        }

        // 建立子线程处理
        let thread_num = 4;
        let mut senders = Vec::new();
        let mut handlers = Vec::new();
        let (main_tx, main_rx) = channel();
        for i_thread in 0..thread_num {
            let tpl_thread = tpl.clone();
            let output_dir_thread = self.output_dir.clone();
            let config_thread = config.clone();
            let main_tx = main_tx.clone();
            // Create a simple streaming channel
            let (tx, rx) = channel();
            senders.push(tx);
            let handler = thread::spawn(move || loop {
                let tup = rx.recv();
                if tup.is_err() {
                    println!("rx droped, i_thread: {:?}", i_thread);
                    break;
                }
                let (_img_path_buf, i_img_path) = tup.unwrap();
                let img_path_buf: PathBuf = _img_path_buf;


                self::draw(&output_dir_thread, &tpl_thread, &img_path_buf, &config_thread);

                println!(
                    "finished file_name: {}, i_img_path: {}, i_thread: {:?}",
                    img_path_buf.to_str().unwrap(),
                    i_img_path,
                    i_thread
                );
                main_tx.send(i_img_path).unwrap();
            });
            handlers.push(handler);
        }
        // 分配任务到线程
        let mut i_img_path = 0;
        for img_path_buf in qrcode_paths {
            let tx = senders.get(i_img_path % thread_num).unwrap();
            let tup = (img_path_buf, i_img_path);
            tx.send(tup).unwrap();

            i_img_path = i_img_path + 1;
        }
        // 等待所有任务完成
        let mut count_finished = 0;
        loop {
            let _ = main_rx.recv().unwrap();
            count_finished = count_finished + 1;
            if count_finished == count {
                break;
            }
        }
        // 关闭tx
        for sender in senders {
            drop(sender);
        }
        // 所有线程join
        for handler in handlers {
            handler.join().unwrap();
        }

        println!("generate qrcode in {}", timer.elapsed().as_millis());
    }
}

fn draw(output_dir: &PathBuf, tpl: &DynamicImage, img_path_buf: &PathBuf, config: &Config) {
    
    let qrcode_name = img_path_buf.file_name().unwrap().to_str().unwrap();
    let qrcode = match image::open(img_path_buf.clone()) {
        Ok(image) => image,
        _ => {
            println!("{} 图片格式有误", qrcode_name);
            panic!();
        }
    };
    let resized_qrcode = qrcode.resize(config.qrcode.width, config.qrcode.height, image::imageops::Triangle);
    
    let mut output_file = output_dir.clone();
    output_file.push(PathBuf::from(qrcode_name));
    let mut tpl_new = tpl.clone();
    tpl_new.copy_from(&resized_qrcode, config.qrcode.x, config.qrcode.y).unwrap();
    tpl_new
        .save(format!("{}", output_file.as_path().to_str().unwrap()))
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

    // #[test]
    // fn test_qrcodes() {
    //     use super::QRCode;
    //     let qrc = QRCode::default();
    //     qrc.qrcodes();
    // }

    #[test]
    fn test_thread() {
        use std::thread;
        use std::time::Duration;

        println!("before call {:?}", thread::current().id());

        let handler = thread::spawn(|| {
            println!("sleep for 3s {:?}", thread::current().id());
            thread::sleep(Duration::from_secs(3));
        });

        println!("after call {:?}", thread::current().id());

        handler.join().unwrap();
    }
}
