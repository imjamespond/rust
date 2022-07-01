// mod foo;
mod foobar; // 不能和foo名称重叠

// use crate::foo::bar::bar;
use ferris_says::say;
// use foo::foo; 
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message: String = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    // foo();
    // bar();
    foobar::foobar();
}
