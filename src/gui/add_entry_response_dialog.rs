use adw::prelude::*;
use relm4::prelude::*;

pub struct AddEntryResponseDialog {
    pub error_text: String,
    is_active: bool,
}

#[derive(Debug)]
pub enum AddEntryResponseDialogMsg {
    AddEntryFail(String),
    AddEntrySuccess,
}

#[relm4::component(pub)]
impl SimpleComponent for AddEntryResponseDialog {
    type Init = ();
    type Input = AddEntryResponseDialogMsg;
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
        let model = AddEntryResponseDialog {
            error_text: String::new(),
            is_active: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AddEntryResponseDialogMsg::AddEntryFail(error_text) => {
                self.error_text = error_text;
                self.is_active = true;
            }

            AddEntryResponseDialogMsg::AddEntrySuccess => {}
        }
    }
}
