use gtk::prelude::*;
use relm4::{actions::ActionName, prelude::*, Sender};

struct LoginPrompt {
    email: gtk::EntryBuffer,
    password: gtk::EntryBuffer,
}

#[derive(Debug)]
enum LoginMsg {
    Accept,
    Cancel,
}

#[relm4::component]
impl SimpleComponent for LoginPrompt {
    type Init = ();
    type Input = LoginMsg;
    type Output = String;

    view! {
        dialog = libadwaita::ApplicationWindow {
            set_margin_all: 20,
            set_modal: false,
            set_title: Some("Authentication"),
            set_default_size: (500, 300),
            set_css_classes: &["background", "csd"],

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                libadwaita::HeaderBar {
                    set_show_end_title_buttons: true,

                    #[wrap(Some)]
                    set_title_widget = &libadwaita::WindowTitle {
                        set_title: "Authentication"
                    },
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_margin_all: 10,
                    set_spacing: 10,

                    gtk::Label {
                        set_label: "Email"
                    },
                    gtk::Entry {
                        set_buffer: &model.email,
                    },
                    gtk::Label {
                        set_label: "Password"
                    },
                    gtk::Entry {
                        set_buffer: &model.password,
                    },
                    gtk::Button {
                        set_margin_all: 40,
                        set_label: "Login",
                        connect_clicked => LoginMsg::Accept
                    }
                },
            },
        },
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = LoginPrompt {
            email: gtk::EntryBuffer::default(),
            password: gtk::EntryBuffer::default(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            LoginMsg::Accept => {
                //sender.output(self.email.text().into()).unwrap();
                //sender.output(self.password.text().into()).unwrap();
                display_input(self.email.text().into(), self.password.text().into());
            }
            LoginMsg::Cancel => {
                sender.output(String::default()).unwrap();
            }
        }
    }

    fn shutdown(&mut self, _widgets: &mut Self::Widgets, _output: Sender<Self::Output>) {
        println!("LoginPrompt shutdown");
    }
}

pub fn run_login_prompt() {
    let login_prompt = RelmApp::new("login_prompt");
    login_prompt.run::<LoginPrompt>(());
}

pub fn display_input(email: String, password: String) {
    println!("{} {}", email, password);
}
