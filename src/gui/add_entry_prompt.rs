use std::{cell::RefCell, rc::Rc};

use super::{
    actions::*, add_entry_response_dialog::AddEntryResponseDialog, entry_list_item::EntryListItem,
};
use crate::AppState;

use super::main_window::EntryTypeView;
use adw::prelude::*;
use relm4::{component::Connector, prelude::*};

pub struct AddPassword {
    name: gtk::EntryBuffer,
    username: gtk::EntryBuffer,
    password: gtk::EntryBuffer,
    url: gtk::EntryBuffer,
    expiration_date: gtk::EntryBuffer,
}

pub struct AddNote {
    name: gtk::EntryBuffer,
    content: gtk::EntryBuffer,
}

pub struct AddCard {
    name: gtk::EntryBuffer,
    cardholder_name: gtk::EntryBuffer,
    card_number: gtk::EntryBuffer,
    security_code: gtk::EntryBuffer,
    expiration_date: gtk::EntryBuffer,
}

pub struct AddEntryPrompt {
    is_active: bool,

    entry_type_view: EntryTypeView,

    add_password: AddPassword,
    add_note: AddNote,
    add_card: AddCard,

    pub response_dialog: Connector<AddEntryResponseDialog>,

    pub app_state: Rc<RefCell<AppState>>,
}

#[derive(Debug)]
pub enum AddEntryPromptMsg {
    SetMode(EntryTypeView),

    AddPress,

    Show,
}

#[derive(Debug)]
pub enum AddEntryPromptOutput {
    NewEntryListItem(EntryListItem),
}

#[relm4::component(pub)]
impl SimpleComponent for AddEntryPrompt {
    type Init = Rc<RefCell<AppState>>;
    type Input = AddEntryPromptMsg;
    type Output = AddEntryPromptOutput;

    view! {
        adw::ApplicationWindow {
            set_title: Some("Add Entry"),
            set_modal: true,
            set_css_classes: &["background", "csd"],
            set_hide_on_close: true,

            #[watch]
            set_visible: model.is_active,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                adw::HeaderBar {
                    set_show_end_title_buttons: true,

                    #[wrap(Some)]
                    set_title_widget = &gtk::Box {
                        add_css_class: "linked",
                        append: group = &gtk::ToggleButton {
                            set_label: "Passwords",
                            set_has_frame: true,
                            set_active: true,
                            connect_clicked[sender] => move |_| {
                                sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::Password));

                            },
                        },
                        gtk::ToggleButton {
                            set_label: "Notes",
                            set_has_frame: true,
                            set_group: Some(&group),
                            connect_clicked[sender] => move |_| {
                                sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::Note));
                            }
                        },

                        gtk::ToggleButton {
                            set_label: "Cards",
                            set_has_frame: true,
                            set_group: Some(&group),
                            connect_clicked[sender] => move |_| {
                                sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::Card));
                            }
                        },

                        gtk::ToggleButton {
                            set_label: "OTP",
                            set_has_frame: true,
                            set_group: Some(&group),
                            connect_clicked[sender] => move |_| {
                                sender.input(AddEntryPromptMsg::SetMode(EntryTypeView::TOTP));
                            }
                        },
                    },
                },

                // Add Password Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::Password),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.name,
                    },

                    gtk::Label {
                        set_label: "Username",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.username,
                    },

                    gtk::Label {
                        set_label: "Password",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.password,
                    },

                    gtk::Label {
                        set_label: "URL",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.url,
                    },

                    gtk::Label {
                        set_label: "Expiration Date",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_password.expiration_date,
                    },
                },

                // Add Note Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::Note),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_note.name,
                    },

                    gtk::Label {
                        set_label: "Content",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_note.content,
                    },
                },

                // Add Card Box
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,

                    #[watch]
                    set_visible: matches!(model.entry_type_view, EntryTypeView::Card),

                    gtk::Label {
                        set_label: "Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.name,
                    },

                    gtk::Label {
                        set_label: "Cardholder Name",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.cardholder_name,
                    },

                    gtk::Label {
                        set_label: "Card Number",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.card_number,
                    },

                    gtk::Label {
                        set_label: "Security Code",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.security_code,
                    },

                    gtk::Label {
                        set_label: "Expiration Date",
                    },
                    gtk::Entry {
                        set_buffer: &model.add_card.expiration_date,
                    },
                },

                gtk::Button {
                    set_margin_all: 40,
                    set_label: "Add",
                    connect_clicked[sender] => move |_| {
                        sender.input(AddEntryPromptMsg::AddPress);
                    }
                }
            }
        }
    }

    fn init(
        state: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AddEntryPrompt {
            is_active: false,

            entry_type_view: EntryTypeView::Password,

            add_password: AddPassword {
                name: gtk::EntryBuffer::default(),
                username: gtk::EntryBuffer::default(),
                password: gtk::EntryBuffer::default(),
                url: gtk::EntryBuffer::default(),
                expiration_date: gtk::EntryBuffer::default(),
            },
            add_note: AddNote {
                name: gtk::EntryBuffer::default(),
                content: gtk::EntryBuffer::default(),
            },
            add_card: AddCard {
                name: gtk::EntryBuffer::default(),
                cardholder_name: gtk::EntryBuffer::default(),
                card_number: gtk::EntryBuffer::default(),
                security_code: gtk::EntryBuffer::default(),
                expiration_date: gtk::EntryBuffer::default(),
            },

            response_dialog: AddEntryResponseDialog::builder()
                .transient_for(&root)
                .launch(()),

            app_state: state,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        match msg {
            AddEntryPromptMsg::SetMode(mode) => {
                self.entry_type_view = mode;
            }

            AddEntryPromptMsg::AddPress => match self.entry_type_view {
                EntryTypeView::Password => {
                    let name = self.add_password.name.text();
                    let username = self.add_password.username.text();
                    let password = self.add_password.password.text();
                    let url = self.add_password.url.text();
                    let expiration_date = self.add_password.expiration_date.text();

                    if let Ok(new_entry_list_item) = add_password_action(
                        &name,
                        &username,
                        &password,
                        &url,
                        &expiration_date,
                        self,
                    ) {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }

                EntryTypeView::Note => {
                    let name = self.add_note.name.text();
                    let content = self.add_note.content.text();

                    if let Ok(new_entry_list_item) = add_note_action(&name, &content, self) {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }

                EntryTypeView::Card => {
                    let name = self.add_card.name.text();
                    let cardholder_name = self.add_card.cardholder_name.text();
                    let card_number = self.add_card.card_number.text();
                    let security_code = self.add_card.security_code.text();
                    let expiration_date = self.add_card.expiration_date.text();

                    if let Ok(new_entry_list_item) = add_card_action(
                        &name,
                        &cardholder_name,
                        &card_number,
                        &security_code,
                        &expiration_date,
                        self,
                    ) {
                        sender
                            .output(AddEntryPromptOutput::NewEntryListItem(new_entry_list_item))
                            .unwrap();
                    }
                }

                _ => {}
            },

            AddEntryPromptMsg::Show => {
                self.is_active = true;
            }
        }
    }
}
