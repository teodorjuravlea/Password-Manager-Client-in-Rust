use crate::model::DataVault;
use crate::requests::{login_request, register_request};
use crate::AppState;
use adw::prelude::*;
use relm4::{component::Connector, prelude::*, Sender};
use std::rc::Rc;

pub struct ErrorDialog {
    pub error_text: String,
    is_active: bool,
}

#[derive(Debug)]
pub enum ErrorDialogMsg {
    LoginFail(String),

    RegisterSuccess,
    RegisterFail(String),
}

#[relm4::component(pub)]
impl SimpleComponent for ErrorDialog {
    type Init = ();
    type Input = ErrorDialogMsg;
    type Output = ();

    view! {
        #[name = "dialog"]
        adw::MessageDialog {
            #[watch]
            set_visible: model.is_active,
            #[watch]
            set_heading: Some(&model.error_text),
            add_response: ("close", "Close"),
            set_hide_on_close: true,
        }
    }

    fn init(
        (): Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ErrorDialog {
            error_text: String::new(),
            is_active: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            ErrorDialogMsg::LoginFail(error_text) => {
                self.error_text = error_text;
                self.is_active = true;
            }

            ErrorDialogMsg::RegisterSuccess => {
                self.error_text = "Registration successful".to_string();
                self.is_active = true;
            }

            ErrorDialogMsg::RegisterFail(error_text) => {
                self.error_text = error_text;
                self.is_active = true;
            }
        }
    }
}

#[derive(Debug)]
enum AuthAppMode {
    Login,
    Register,
}

struct AuthPrompt {
    mode: AuthAppMode,

    login_email: gtk::EntryBuffer,
    login_password: gtk::EntryBuffer,

    register_email: gtk::EntryBuffer,
    register_password1: gtk::EntryBuffer,
    register_password2: gtk::EntryBuffer,

    error_dialog: Connector<ErrorDialog>,

    app_state: Rc<AppState>,
}

#[derive(Debug)]
enum AuthMsg {
    SetMode(AuthAppMode),

    LoginPress,
    RegisterPress,
}

#[relm4::component(pub)]
impl SimpleComponent for AuthPrompt {
    type Init = Rc<AppState>;
    type Input = AuthMsg;
    type Output = AuthMsg;

    view! {
        adw::ApplicationWindow {
            set_margin_all: 20,
            set_modal: true,
            set_title: Some("Authentication"),
            set_resizable: false,
            set_default_size: (500, 300),
            set_css_classes: &["background", "csd"],

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,

                    #[wrap(Some)]
                    set_title_widget = &gtk::Box {
                        add_css_class: "linked",
                        append: group = &gtk::ToggleButton {
                            set_label: "Login",
                            set_has_frame: true,
                            set_active: true,
                            connect_clicked[sender] => move |_| {
                                sender.input(AuthMsg::SetMode(AuthAppMode::Login));
                            },
                        },
                        gtk::ToggleButton {
                            set_label: "Register",
                            set_has_frame: true,
                            set_group: Some(&group),
                            connect_clicked[sender] => move |_| {
                                sender.input(AuthMsg::SetMode(AuthAppMode::Register));
                            }
                        },
                    },
                },

                // Login Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 10,
                    set_spacing: 10,

                    #[watch]
                    set_visible: match model.mode {
                        AuthAppMode::Login => true,
                        AuthAppMode::Register => false,
                    },

                    gtk::Label {
                        set_label: "Email"
                    },
                    gtk::Entry {
                        set_buffer: &model.login_email,
                    },
                    gtk::Label {
                        set_label: "Password"
                    },
                    gtk::Entry {
                        set_buffer: &model.login_password,
                    },
                    gtk::Button {
                        set_margin_all: 40,
                        set_label: "Login",
                        connect_clicked[sender] => move |_| {
                            sender.input(AuthMsg::LoginPress);
                        }
                    }
                },

                // Register Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 10,
                    set_spacing: 10,

                    #[watch]
                    set_visible: match model.mode {
                        AuthAppMode::Login => false,
                        AuthAppMode::Register => true,
                    },

                    gtk::Label {
                        set_label: "Email"
                    },
                    gtk::Entry {
                        set_buffer: &model.register_email,
                    },
                    gtk::Label {
                        set_label: "Password"
                    },
                    gtk::Entry {
                        set_buffer: &model.register_password1,
                    },
                    gtk::Label {
                        set_label: "Confirm Password"
                    },
                    gtk::Entry {
                        set_buffer: &model.register_password2,
                    },
                    gtk::Button {
                        set_margin_all: 40,
                        set_label: "Register",
                        connect_clicked[sender] => move |_| {
                            sender.input(AuthMsg::RegisterPress);
                        }
                    }
                },
            },
        },
    }

    fn init(
        state: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AuthPrompt {
            app_state: state,

            mode: AuthAppMode::Login,

            login_email: gtk::EntryBuffer::default(),
            login_password: gtk::EntryBuffer::default(),

            register_email: gtk::EntryBuffer::default(),
            register_password1: gtk::EntryBuffer::default(),
            register_password2: gtk::EntryBuffer::default(),

            error_dialog: ErrorDialog::builder().transient_for(&root).launch(()),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AuthMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AuthMsg::LoginPress => {
                let email = self.login_email.text();
                let password = self.login_password.text();

                match login_request(
                    &email,
                    &password,
                    &self.app_state.client,
                    &self.app_state.base_url,
                ) {
                    Ok(response) => {
                        println!("Login successful: {}", response.status);

                        if let Some(app_state) = Rc::get_mut(&mut self.app_state) {
                            app_state.is_logged_in = true;

                            let data_vault = match DataVault::new(&email, &password) {
                                Ok(data_vault) => data_vault,
                                Err(e) => {
                                    panic!("Error creating data vault: {}", e);
                                }
                            };

                            app_state.vault = Some(data_vault);
                        }
                    }
                    Err(e) => {
                        println!("Login failed: {}", e);

                        self.error_dialog
                            .emit(ErrorDialogMsg::LoginFail(e.to_string()));
                    }
                }
            }

            AuthMsg::RegisterPress => {
                let email = self.register_email.text();
                let password1 = self.register_password1.text();
                let password2 = self.register_password2.text();

                match register_request(
                    &email,
                    &password1,
                    &password2,
                    &self.app_state.client,
                    &self.app_state.base_url,
                ) {
                    Ok(response) => {
                        println!("Register successful: {}", response.status);

                        self.error_dialog.emit(ErrorDialogMsg::RegisterSuccess);
                    }
                    Err(e) => {
                        println!("Register failed: {}", e);

                        self.error_dialog
                            .emit(ErrorDialogMsg::RegisterFail(e.to_string()));
                    }
                }
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: Sender<Self::Output>) {
        println!("AuthPrompt shutdown");
    }
}

pub fn run_login_prompt(state: Rc<AppState>) {
    let login_prompt = RelmApp::new("login_prompt");
    login_prompt.run::<AuthPrompt>(state);
}
