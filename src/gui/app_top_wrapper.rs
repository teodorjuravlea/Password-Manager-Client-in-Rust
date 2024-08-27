use std::{cell::RefCell, rc::Rc};

use adw::prelude::*;
use relm4::{prelude::*, Controller, SimpleComponent};

use crate::{
    entries::fill_data_vault_from_response, requests::get_all_encrypted_data_entries_request,
    AppState,
};

use super::{
    auth_prompt::{AuthPrompt, LoggedInMsg},
    main_window::{LoggedOutMsg, MainWindow},
};

pub struct AppTopWrapper {
    auth_prompt: Option<Controller<AuthPrompt>>,
    main_window: Option<Controller<MainWindow>>,

    app_state: Rc<RefCell<AppState>>,
}

#[derive(Debug)]
pub enum AppTopWrapperInput {
    LoggedIn,
    LoggedOut,
}

#[relm4::component(pub)]
impl SimpleComponent for AppTopWrapper {
    type Init = Rc<RefCell<AppState>>;
    type Input = AppTopWrapperInput;
    type Output = ();

    view! {
        adw::ApplicationWindow {
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppTopWrapper {
            auth_prompt: Some(AuthPrompt::builder().launch(init.clone()).forward(
                sender.input_sender(),
                |msg| match msg {
                    LoggedInMsg::LoggedIn => AppTopWrapperInput::LoggedIn,
                },
            )),
            main_window: None,

            app_state: init,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AppTopWrapperInput::LoggedIn => {
                let encrypted_entries_response = match get_all_encrypted_data_entries_request(
                    &self.app_state.borrow().client,
                    &self.app_state.borrow().base_url,
                ) {
                    Ok(encrypted_entries) => encrypted_entries,
                    Err(e) => {
                        println!("Failed to get encrypted entries: {}", e);
                        return;
                    }
                };

                // Fill the data vault
                match self.app_state.borrow_mut().vault.as_mut() {
                    Some(vault) => {
                        fill_data_vault_from_response(vault, encrypted_entries_response);
                    }
                    None => {
                        println!("Failed to get mutable reference to data vault");
                        return;
                    }
                };

                self.auth_prompt = None;
                self.main_window = Some(
                    MainWindow::builder()
                        .launch(self.app_state.clone())
                        .forward(sender.input_sender(), |msg| match msg {
                            LoggedOutMsg::LoggedOut => AppTopWrapperInput::LoggedOut,
                        }),
                );
            }

            AppTopWrapperInput::LoggedOut => {
                self.main_window = None;

                self.app_state.borrow_mut().vault = None;

                self.auth_prompt = Some(
                    AuthPrompt::builder()
                        .launch(self.app_state.clone())
                        .forward(sender.input_sender(), |msg| match msg {
                            LoggedInMsg::LoggedIn => AppTopWrapperInput::LoggedIn,
                        }),
                );
            }
        }
    }
}
