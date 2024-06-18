use std::{cell::RefCell, rc::Rc};

use relm4::{prelude::*, typed_view::list::TypedListView};

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
