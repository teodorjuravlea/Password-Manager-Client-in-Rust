use adw::prelude::*;
use relm4::{prelude::*, typed_view::list::RelmListItem};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryType {
    Password,
    Note,
    Card,
    TOTP,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryListItem {
    pub name: String,
    pub subtext: String,
    pub entry_type: EntryType,
    pub is_selected: bool,
}

impl EntryListItem {
    pub fn new(name: &str, subtext: &str, entry_type: EntryType) -> EntryListItem {
        EntryListItem {
            name: name.to_string(),
            subtext: subtext.to_string(),
            entry_type,
            is_selected: false,
        }
    }
}

pub struct Widgets {
    label1: gtk::Label,
    label2: gtk::Label,
}

impl RelmListItem for EntryListItem {
    type Root = gtk::Box;
    type Widgets = Widgets;

    fn setup(_item: &gtk::ListItem) -> (gtk::Box, Widgets) {
        relm4::view! {
            entry_box = gtk::Box {
                set_height_request: 50,
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                #[name = "name"]
                gtk::Label{
                    set_use_markup: true,
                    set_single_line_mode: true,
                    set_halign: gtk::Align::Start,
                    set_valign: gtk::Align::Start,
                },

                #[name = "subtext"]
                gtk::Label{
                    set_use_markup: true,
                    set_single_line_mode: true,
                    set_halign: gtk::Align::Start,
                    set_valign: gtk::Align::Start,
                },
            }
        }

        let widgets = Widgets {
            label1: name,
            label2: subtext,
        };

        (entry_box, widgets)
    }

    fn bind(&mut self, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        let Widgets {
            label1: name,
            label2: subtext,
        } = widgets;

        name.set_label(format!("<big><b>{}</b></big>", self.name).as_str());
        subtext.set_label(self.subtext.as_str());
    }
}
