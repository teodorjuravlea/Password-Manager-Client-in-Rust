use std::{cell::RefCell, rc::Rc};

use adw::glib::random_set_seed;
use relm4::{prelude::*, typed_view::list::TypedListView};

use crate::model::{Card, EntriesVault, Note, Password, TOTPEntry};
use crate::totp::generate_totp_token;
use crate::AppState;

use super::entry_list_item::{EntryListItem, EntryType};

pub fn make_list_view_wrapper_from_data_vault(
    state: Rc<RefCell<AppState>>,
) -> TypedListView<EntryListItem, gtk::SingleSelection> {
    match state.borrow().vault.as_ref() {
        Some(data_vault) => {
            let mut list_view_wrapper: TypedListView<EntryListItem, gtk::SingleSelection> =
                TypedListView::new();

            for password in data_vault.entries_vault.passwords.iter() {
                list_view_wrapper.append(EntryListItem::new(
                    &password.name,
                    &password.username,
                    EntryType::Password,
                ));
            }

            for note in data_vault.entries_vault.notes.iter() {
                list_view_wrapper.append(EntryListItem::new(&note.name, "", EntryType::Note));
            }

            for card in data_vault.entries_vault.cards.iter() {
                list_view_wrapper.append(EntryListItem::new(&card.name, "", EntryType::Card));
            }

            for totp_entry in data_vault.entries_vault.totp_entries.iter() {
                list_view_wrapper.append(EntryListItem::new(&totp_entry.name, "", EntryType::TOTP));
            }

            list_view_wrapper
        }
        None => {
            panic!("Failed to get reference to data vault");
        }
    }
}

pub fn get_list_view_item_index(
    name: &str,
    content_type: &str,
    list_view_wrapper: &TypedListView<EntryListItem, gtk::SingleSelection>,
) -> Result<u32, String> {
    let entry_type = match content_type {
        "password" => EntryType::Password,
        "note" => EntryType::Note,
        "card" => EntryType::Card,
        "totp" => EntryType::TOTP,
        _ => panic!("Invalid entry type"),
    };

    let mut i = 0;

    while i < list_view_wrapper.len() {
        if let Some(list_item) = list_view_wrapper.get(i) {
            let list_item = list_item.borrow();

            if list_item.name == name && list_item.entry_type == entry_type {
                return Ok(i);
            }
        }

        i += 1;
    }

    Err("Failed to get list view item index".to_string())
}

pub struct ActiveEntriesData {
    pub entries_vault: EntriesVault,

    pub active_password_data: Option<Password>,
    pub active_note_data: Option<Note>,
    pub active_card_data: Option<Card>,
    pub active_totp_data: Option<TOTPEntry>,

    pub current_totp_token: Option<String>,
}

impl ActiveEntriesData {
    pub fn update_vault_data(&mut self, state: Rc<RefCell<AppState>>) {
        match state.borrow().vault.as_ref() {
            Some(data_vault) => {
                self.entries_vault = data_vault.entries_vault.clone();
            }
            None => {
                panic!("Failed to get reference to data vault");
            }
        }
    }

    pub fn set_active_index(&mut self, view: u8, index: u32) {
        match view {
            0 => {
                self.active_password_data =
                    Some(self.entries_vault.passwords[index as usize].clone());
            }
            1 => {
                self.active_note_data = Some(self.entries_vault.notes[index as usize].clone());
            }
            2 => {
                self.active_card_data = Some(self.entries_vault.cards[index as usize].clone());
            }
            3 => {
                self.active_totp_data =
                    Some(self.entries_vault.totp_entries[index as usize].clone());

                self.update_current_totp_token();
            }
            _ => panic!("Invalid view index"),
        }
    }

    pub fn update_current_totp_token(&mut self) {
        self.current_totp_token =
            Some(generate_totp_token(self.active_totp_data.clone().unwrap()).unwrap());
    }
}

pub fn make_active_entries_data(state: Rc<RefCell<AppState>>) -> ActiveEntriesData {
    match state.borrow().vault.as_ref() {
        Some(data_vault) => ActiveEntriesData {
            entries_vault: data_vault.entries_vault.clone(),

            active_password_data: None,
            active_note_data: None,
            active_card_data: None,
            active_totp_data: None,

            current_totp_token: None,
        },
        None => {
            panic!("Failed to get reference to data vault");
        }
    }
}

pub fn generate_random_password() -> String {
    let charset = random_string::charsets::ALPHANUMERIC.to_string() + "!@#$%^&*()_+-=";

    random_string::generate_rng(12..16, charset)
}
