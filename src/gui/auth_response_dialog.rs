use adw::prelude::*;
use relm4::prelude::*;

pub struct AuthResponseDialog {
    pub error_text: String,
    is_active: bool,
}

#[derive(Debug)]
pub enum AuthResponseDialogMsg {
    LoginFail(String),

    RegisterSuccess,
    RegisterFail(String),
}

#[relm4::component(pub)]
impl SimpleComponent for AuthResponseDialog {
    type Init = ();
    type Input = AuthResponseDialogMsg;
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
        let model = AuthResponseDialog {
            error_text: String::new(),
            is_active: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AuthResponseDialogMsg::LoginFail(error_text) => {
                self.error_text = error_text;
                self.is_active = true;
            }

            AuthResponseDialogMsg::RegisterSuccess => {
                self.error_text = "Registration successful".to_string();
                self.is_active = true;
            }

            AuthResponseDialogMsg::RegisterFail(error_text) => {
                self.error_text = error_text;
                self.is_active = true;
            }
        }
    }
}
