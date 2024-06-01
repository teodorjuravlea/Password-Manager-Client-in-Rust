use crate::{gui, AppState};

pub fn test_gui(state: AppState) {
    gui::authentication::run_login_prompt(state);
}
