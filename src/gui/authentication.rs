use crate::requests::{login_request, register_request};
use crate::AppState;
use gtk::prelude::*;
use relm4::{prelude::*, Sender};

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

    app_state: AppState,
}

#[derive(Debug)]
enum AuthMsg {
    SetMode(AuthAppMode),
    LoginPress,
    RegisterPress,
}

#[relm4::component]
impl SimpleComponent for AuthPrompt {
    type Init = AppState;
    type Input = AuthMsg;
    type Output = AuthMsg;

    view! {
        auth_window = libadwaita::ApplicationWindow {
            set_margin_all: 20,
            set_modal: true,
            set_title: Some("Authentication"),
            set_resizable: false,
            set_default_size: (500, 300),
            set_css_classes: &["background", "csd"],

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                libadwaita::HeaderBar {
                    set_show_end_title_buttons: true,

                    #[wrap(Some)]
                    set_title_widget = &gtk::Box {
                        add_css_class: "linked",
                        append: group = &gtk::ToggleButton {
                            set_label: "Login",
                            set_has_frame: true,
                            set_active: true,
                            connect_clicked => AuthMsg::SetMode(AuthAppMode::Login),
                        },
                        gtk::ToggleButton {
                            set_label: "Register",
                            set_has_frame: true,
                            set_group: Some(&group),
                            connect_clicked => AuthMsg::SetMode(AuthAppMode::Register),
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
                        connect_clicked => AuthMsg::LoginPress
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
                        connect_clicked => AuthMsg::RegisterPress
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
                //sender.output(self.email.text().into()).unwrap();
                //sender.output(self.password.text().into()).unwrap();
                display_input(
                    self.login_email.text().into(),
                    self.login_password.text().into(),
                );

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
                    }
                    Err(e) => {
                        println!("Login failed: {}", e);
                    }
                }
            }

            AuthMsg::RegisterPress => {
                //sender.output(self.email.text().into()).unwrap();
                //sender.output(self.password.text().into()).unwrap();
                display_input(
                    self.register_email.text().into(),
                    self.register_password1.text().into(),
                );

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
                    }
                    Err(e) => {
                        println!("Register failed: {}", e);
                    }
                }
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: Sender<Self::Output>) {
        println!("AuthPrompt shutdown");
    }
}

pub fn run_login_prompt(state: AppState) {
    let login_prompt = RelmApp::new("login_prompt");
    login_prompt.run::<AuthPrompt>(state);
}

pub fn display_input(email: String, password: String) {
    println!("{} {}", email, password);
}
