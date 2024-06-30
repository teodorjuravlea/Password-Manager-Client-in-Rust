use std::{cell::RefCell, rc::Rc};

use relm4::{prelude::*, typed_view::list::TypedListView};

use crate::model::{Card, EntriesVault, Note, Password, TOTPEntry};
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

pub struct ActiveEntriesData {
    pub entries_vault: EntriesVault,

    pub active_password_data: Option<Password>,
    pub active_note_data: Option<Note>,
    pub active_card_data: Option<Card>,
    pub active_totp_entry_data: Option<TOTPEntry>,
}

impl ActiveEntriesData {
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
                self.active_totp_entry_data =
                    Some(self.entries_vault.totp_entries[index as usize].clone());
            }
            _ => panic!("Invalid view index"),
        }
    }
}

pub fn make_active_entries_data(state: Rc<RefCell<AppState>>) -> ActiveEntriesData {
    match state.borrow().vault.as_ref() {
        Some(data_vault) => ActiveEntriesData {
            entries_vault: data_vault.entries_vault.clone(),

            active_password_data: None,
            active_note_data: None,
            active_card_data: None,
            active_totp_entry_data: None,
        },
        None => {
            panic!("Failed to get reference to data vault");
        }
    }
}
