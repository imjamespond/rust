extern crate native_windows_gui as nwg;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use super::basic_app::BasicApp;

pub struct BasicAppUi {
    pub inner: Rc<BasicApp>,
    pub default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl Drop for BasicAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            nwg::unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for BasicAppUi {
    type Target = BasicApp;

    fn deref(&self) -> &BasicApp {
        &self.inner
    }
}
