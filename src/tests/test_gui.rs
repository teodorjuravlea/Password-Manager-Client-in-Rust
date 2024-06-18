use crate::{gui, AppState};
use std::{cell::RefCell, rc::Rc};

pub fn test_gui(state: Rc<RefCell<AppState>>) {
    //gui::auth_prompt::run_login_prompt(state);
    gui::main_window::run_main_window(state);
}
