use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Once;

static SINGLE: Once = Once::new();

pub struct Pdf {
    pub config_file_str: String,
    pub qrcode_dir_str: String,
    pub pdf_tool_dir: bool,
}

impl Default for Pdf {
    fn default() -> Self {
        if SINGLE.is_completed() {
            panic!("Only One Pdf Can Be Created!"); // 避免多线程 call once 调用没完成
        }

        self::set_env();

        let config_file = self::get_abs_path(&["config.json"]);
        let config_file_str = String::from(config_file.to_str().unwrap());

        let qrcode_dir = self::get_abs_path(&["qrcode"]);
        let qrcode_dir_str = String::from(qrcode_dir.to_str().unwrap());

        println!("before once {:?}", std::thread::current().id());
        SINGLE.call_once(|| {
            println!("once {:?}", std::thread::current().id());
        });
        println!("after once {:?}", std::thread::current().id());

        Self {
            config_file_str,
            qrcode_dir_str,
            pdf_tool_dir: false,
        }
    }
}

impl Pdf {
    pub fn exec(&self) {
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

        let mut the_process = Command::new("node")
            .arg("src/run.js")
            .arg(self.config_file_str.to_string())
            .arg(self.qrcode_dir_str.to_string())
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

    pub fn npm_install(&mut self) {
        if cfg!(target_os = "windows") {
            let npm = get_abs_path(&["node", "npm.cmd"]);
            let npmcmd = npm.to_str().unwrap(); //".\\node\\npm.cmd";
            let mut the_process = Command::new("cmd")
                // .current_dir(npmcmd)
                .args(["/C", npmcmd, "install", ".\\pdf-tool-1.0.0.tgz"])
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

            self.set_pdf_tool_root();
        }
    }

    pub fn set_pdf_tool_root(&mut self)  {
        let pdf_tool_path = get_abs_path(&["node_modules", "pdf-tool"]);
        let pdf_tool_root = env::set_current_dir(&pdf_tool_path);

        self.pdf_tool_dir = pdf_tool_root.is_ok();
    }
}

// fn abs_path(path: &str) -> io::Result<PathBuf> {
//     // windows ? 在前，不可用于cmd
//     let pathbuf = PathBuf::from(path);
//     let abspath = fs::canonicalize(&pathbuf);
//     abspath
// }

pub(crate) fn get_abs_path(paths: &[&str]) -> PathBuf {
    let mut absolute_path = env::current_dir().unwrap();
    for path in paths {
        let pathbuf = PathBuf::from(path);
        absolute_path.push(pathbuf);
    }
    absolute_path
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
    let node = get_abs_path(&["node"]);
    let nodepath = node.to_str().unwrap();
    append_path(nodepath).unwrap();

    let key = "PATH";
    match std::env::var_os(key) {
        Some(val) => println!("{}: {:?}", key, val),
        None => println!("{} is not defined in the environment.", key),
    };
}

mod tests {

    #[test]
    fn test_once() {
        use super::Pdf;

        let _pdf1 = Pdf::default();
        let _pdf2 = Pdf::default();
    }

    #[test]
    fn test_exec() {
        use super::Pdf;

        let pdf1 = Pdf::default();
        pdf1.exec();
    }

    #[test]
    fn test_path() {
        // let abspath = super::abs_path("./config.json").unwrap();
        let abspath = super::get_abs_path(&["config.json"]);
        println!("{}", abspath.to_str().unwrap());
        let abspath1 = super::get_abs_path(&["qrcode", "input"]);
        println!("{}", abspath1.to_str().unwrap());
    }

    #[test]
    fn test_env() {
        super::set_env();
    }

    #[test]
    fn test_npm_install() {
        use super::Pdf;
        super::set_env();
        let mut pdf1 = Pdf::default();
        pdf1.npm_install();
    }
}
