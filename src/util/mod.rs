use std::{path::PathBuf, env};

pub mod thread;

pub fn get_abs_path(paths: &[&str]) -> PathBuf {
    let mut absolute_path = env::current_dir().unwrap();
    for path in paths {
        let pathbuf = PathBuf::from(path);
        absolute_path.push(pathbuf);
    }
    absolute_path
}