use super::actions::{login_action, register_action};
use super::auth_response_dialog::AuthResponseDialog;
use crate::AppState;
use adw::prelude::*;
use relm4::{component::Connector, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum AuthAppMode {
    Login,
    Register,
}

pub struct AuthPrompt {
    mode: AuthAppMode,

    login_email: gtk::EntryBuffer,
    login_password: gtk::EntryBuffer,

    register_email: gtk::EntryBuffer,
    register_password1: gtk::EntryBuffer,
    register_password2: gtk::EntryBuffer,

    pub response_dialog: Connector<AuthResponseDialog>,

    pub app_state: Rc<RefCell<AppState>>,
}

#[derive(Debug)]
pub enum AuthMsg {
    SetMode(AuthAppMode),

    LoginPress,
    RegisterPress,
}

#[derive(Debug)]
pub enum LoggedInMsg {
    LoggedIn,
}

#[relm4::component(pub)]
impl SimpleComponent for AuthPrompt {
    type Init = Rc<RefCell<AppState>>;
    type Input = AuthMsg;
    type Output = LoggedInMsg;

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
                        set_input_purpose: gtk::InputPurpose::Password,
                        set_visibility: false,
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
                        set_input_purpose: gtk::InputPurpose::Password,
                        set_visibility: false,
                    },
                    gtk::Label {
                        set_label: "Confirm Password"
                    },
                    gtk::Entry {
                        set_buffer: &model.register_password2,
                        set_input_purpose: gtk::InputPurpose::Password,
                        set_visibility: false,
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

            response_dialog: AuthResponseDialog::builder()
                .transient_for(&root)
                .launch(()),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AuthMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AuthMsg::LoginPress => {
                let email = self.login_email.text();
                let password = self.login_password.text();

                if login_action(&email, &password, self).is_ok() {
                    sender.output(LoggedInMsg::LoggedIn).unwrap();
                }
            }

            AuthMsg::RegisterPress => {
                let email = self.register_email.text();
                let password1 = self.register_password1.text();
                let password2 = self.register_password2.text();

                register_action(&email, &password1, &password2, self);
            }
        }
    }
}

pub fn run_auth_prompt(state: Rc<RefCell<AppState>>) {
    let auth_prompt = RelmApp::new("auth_prompt");
    auth_prompt.run::<AuthPrompt>(state);
}
