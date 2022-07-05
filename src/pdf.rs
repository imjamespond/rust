use std::env;
use std::fs;
use std::io;
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

    self::set_env();

    let config_file = self::abs_path("./config.json").unwrap();
    let config_file_str = config_file.to_str().unwrap();

    let qrcode_dir = self::abs_path("./qrcode").unwrap();
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

fn abs_path(path: &str) -> io::Result<PathBuf> {
    let pathbuf = PathBuf::from(path);
    let abspath = fs::canonicalize(&pathbuf);
    abspath
}

fn append_path(addpath: &str) -> Result<(), env::JoinPathsError> {
    if let Some(path) = env::var_os("PATH") {
        let mut paths = env::split_paths(&path).collect::<Vec<_>>();
        paths.push(PathBuf::from(addpath));
        let new_path = env::join_paths(paths)?;
        env::set_var("PATH", &new_path);
    }

    Ok(())
}

fn set_env() {
    let node = abs_path("./node");
    if !node.is_ok() {
        println!("path is not existed.");
        return ;
    }
    let nodepath = node.unwrap();
    append_path(nodepath.to_str().unwrap()).unwrap();

    let key = "PATH";
    match std::env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    };
}

pub fn npm_install(){
    let mut the_process = Command::new("npm")
        .arg("install")
        .arg("pdf-tool-1.0.0.tgz") 
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

mod tests {
    use std::os;

    #[test]
    fn test_exec() {
        use super::exec;

        exec();
    }

    #[test]
    fn test_path() {
        let abspath = super::abs_path("./config.json").unwrap();
        println!("{}", abspath.to_str().unwrap());
    }

    #[test]
    fn test_env() {
        super::set_env();
    }

}
