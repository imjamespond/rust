use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn exec() {
    // let output = if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //             .args(["/C", "echo hello"])
    //             .output()
    //             .expect("failed to execute process")
    // } else {
    //     Command::new("sh")
    //             .arg("-c")
    //             .arg("echo hello")
    //             .output()
    //             .expect("failed to execute process")
    // };

    // let hello = output.stdout;
    // print!("{:?}", String::from_utf8_lossy(&hello));

    let config_file = self::abs_path("./config.json");
    let config_file_str = config_file.to_str().unwrap();

    let qrcode_dir = self::abs_path("./qrcode");
    let qrcode_dir_str = qrcode_dir.to_str().unwrap();

    let root = Path::new("./pdf-tool");
    env::set_current_dir(&root).unwrap();

    let mut the_process = Command::new("node")
        .arg("src/run.js")
        .arg(config_file_str)
        .arg(qrcode_dir_str)
        .spawn()
        .ok()
        .expect("Failed to execute.");

    // Wait for the process to exit.
    match the_process.wait() {
        Ok(status) => println!("Finished, status of {}", status),
        Err(e) => println!("Failed, error: {}", e),
    }

    // Get the PID of the process.
    println!("The PID is: {}", the_process.id());
}

fn abs_path(path: &str) -> PathBuf {
    let pathbuf = PathBuf::from(path);
    let abspath = fs::canonicalize(&pathbuf).unwrap();
    abspath
}

mod tests {
    #[test]
    fn test_exec() {
        use super::exec;

        exec();
    }

    #[test]
    fn test_path() {
        let abspath = super::abs_path("./config.json");
        println!("{}", abspath.to_str().unwrap());
    }
}
