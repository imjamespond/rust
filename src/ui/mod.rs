/*!
    A very simple application that show your name in a message box.
    See `basic_d` for the derive version
*/

extern crate native_windows_gui as nwg;

use basic_app::BasicApp;
use nwg::NativeUi;

mod basic_app;
mod basic_app_ui;

pub fn run() {
    nwg::init().expect("Failed to init Native Windows GUI");
    //nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    {
        let mut font = nwg::Font::default();
        nwg::Font::builder()
            .family("Segoe UI")
            .size(18)
            .build(&mut font).expect("Failed to set default font");
        nwg::Font::set_global_default(Some(font));
    }
    let _ui = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
