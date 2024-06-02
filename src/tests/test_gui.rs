use crate::{gui, AppState};
use std::rc::Rc;

pub fn test_gui(state: Rc<AppState>) {
    gui::authentication::run_login_prompt(state);
}
