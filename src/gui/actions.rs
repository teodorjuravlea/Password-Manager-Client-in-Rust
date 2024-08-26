use std::cell::RefCell;
use std::rc::Rc;

use relm4::ComponentController;

use super::add_entry_prompt::AddEntryPrompt;
use super::auth_prompt::AuthPrompt;
use super::auth_response_dialog::AuthResponseDialogMsg;
use super::entry_list_item::EntryListItem;
use crate::gui::add_entry_response_dialog::AddEntryResponseDialogMsg;
use crate::gui::entry_list_item::EntryType;
use crate::model::DataVault;
use crate::requests::*;
use crate::{entries::*, AppState};

pub fn login_action(
    email: &str,
    password: &str,
    auth_prompt: &mut AuthPrompt,
) -> Result<(), String> {
    let mut app_state = auth_prompt.app_state.borrow_mut();

    match login_request(email, password, &app_state.client, &app_state.base_url) {
        Ok(response) => {
            println!("Login successful: {}", response.status);

            app_state.is_logged_in = true;

            let data_vault = match DataVault::new(email, password) {
                Ok(data_vault) => data_vault,
                Err(e) => {
                    panic!("Error creating data vault: {}", e);
                }
            };

            app_state.vault = Some(data_vault);

            Ok(())
        }
        Err(e) => {
            auth_prompt
                .response_dialog
                .emit(AuthResponseDialogMsg::LoginFail(e.to_string()));

            Err(format!("Login failed: {}", e))
        }
    }
}

pub fn register_action(
    email: &str,
    password1: &str,
    password2: &str,
    auth_prompt: &mut AuthPrompt,
) {
    let app_state = auth_prompt.app_state.borrow();

    match register_request(
        email,
        password1,
        password2,
        &app_state.client,
        &app_state.base_url,
    ) {
        Ok(response) => {
            println!("Register successful: {}", response.status);

            auth_prompt
                .response_dialog
                .emit(AuthResponseDialogMsg::RegisterSuccess);
        }
        Err(e) => {
            println!("Register failed: {}", e);

            auth_prompt
                .response_dialog
                .emit(AuthResponseDialogMsg::RegisterFail(e.to_string()));
        }
    }
}

pub fn add_password_action(
    name: &str,
    username: &str,
    password: &str,
    url: &str,
    expiration_date: &str,
    add_entry_prompt: &mut AddEntryPrompt,
) -> Result<EntryListItem, String> {
    let entry = create_password_entry(name, username, password, url, expiration_date);

    let mut app_state = add_entry_prompt.app_state.borrow_mut();

    if let Some(vault) = &app_state.vault {
        let encrypted_entry = match encrypt_password_entry(&entry, &vault.ciphers.password_cipher) {
            Ok(encrypted_entry) => encrypted_entry,
            Err(e) => {
                panic!("Failed to encrypt entry: {}", e);
            }
        };

        match add_encrypted_data_entry_request(
            encrypted_entry,
            &app_state.client,
            &app_state.base_url,
        ) {
            Ok(response) => {
                println!("Add password entry successful: {}", response.status);

                let data_vault = match app_state.vault.as_mut() {
                    Some(vault) => vault,
                    None => {
                        panic!("Failed to get reference to data vault");
                    }
                };

                let entries_vault = &mut data_vault.entries_vault;
                entries_vault.passwords.push(entry);

                return Ok(EntryListItem::new(name, username, EntryType::Password));
            }
            Err(e) => {
                println!("Add password entry failed: {}", e);

                add_entry_prompt
                    .response_dialog
                    .emit(AddEntryResponseDialogMsg::AddEntryFail(e.to_string()));

                return Err(e.to_string());
            }
        }
    }

    panic!("Failed to get reference to app state");
}

pub fn add_note_action(
    name: &str,
    content: &str,
    add_entry_prompt: &mut AddEntryPrompt,
) -> Result<EntryListItem, String> {
    let entry = create_note_entry(name, content);

    let mut app_state = add_entry_prompt.app_state.borrow_mut();

    if let Some(vault) = &app_state.vault {
        let encrypted_entry = match encrypt_note_entry(&entry, &vault.ciphers.note_cipher) {
            Ok(encrypted_entry) => encrypted_entry,
            Err(e) => {
                panic!("Failed to encrypt entry: {}", e);
            }
        };

        match add_encrypted_data_entry_request(
            encrypted_entry,
            &app_state.client,
            &app_state.base_url,
        ) {
            Ok(response) => {
                println!("Add note entry successful: {}", response.status);

                let data_vault = match app_state.vault.as_mut() {
                    Some(vault) => vault,
                    None => {
                        panic!("Failed to get reference to data vault");
                    }
                };

                let entries_vault = &mut data_vault.entries_vault;
                entries_vault.notes.push(entry);

                return Ok(EntryListItem::new(name, "", EntryType::Note));
            }
            Err(e) => {
                println!("Add note entry failed: {}", e);

                add_entry_prompt
                    .response_dialog
                    .emit(AddEntryResponseDialogMsg::AddEntryFail(e.to_string()));

                return Err(e.to_string());
            }
        }
    }

    panic!("Failed to get reference to app state");
}

pub fn add_card_action(
    name: &str,
    cardholder_name: &str,
    card_number: &str,
    security_code: &str,
    expiration_date: &str,
    add_entry_prompt: &mut AddEntryPrompt,
) -> Result<EntryListItem, String> {
    let entry = create_card_entry(
        name,
        cardholder_name,
        card_number,
        security_code,
        expiration_date,
    );

    let mut app_state = add_entry_prompt.app_state.borrow_mut();

    if let Some(vault) = &app_state.vault {
        let encrypted_entry = match encrypt_card_entry(&entry, &vault.ciphers.card_cipher) {
            Ok(encrypted_entry) => encrypted_entry,
            Err(e) => {
                panic!("Failed to encrypt entry: {}", e);
            }
        };

        match add_encrypted_data_entry_request(
            encrypted_entry,
            &app_state.client,
            &app_state.base_url,
        ) {
            Ok(response) => {
                println!("Add card entry successful: {}", response.status);

                let data_vault = match app_state.vault.as_mut() {
                    Some(vault) => vault,
                    None => {
                        panic!("Failed to get reference to data vault");
                    }
                };

                let entries_vault = &mut data_vault.entries_vault;
                entries_vault.cards.push(entry);

                return Ok(EntryListItem::new(name, "", EntryType::Card));
            }
            Err(e) => {
                println!("Add card entry failed: {}", e);

                add_entry_prompt
                    .response_dialog
                    .emit(AddEntryResponseDialogMsg::AddEntryFail(e.to_string()));

                return Err(e.to_string());
            }
        }
    }

    panic!("Failed to get reference to app state");
}

pub fn delete_entry_action(
    name: &str,
    content_type: &str,
    app_state: Rc<RefCell<AppState>>,
) -> Result<(), String> {
    let mut app_state = app_state.borrow_mut();

    match delete_encrypted_data_entry_request(
        name,
        content_type,
        &app_state.client,
        &app_state.base_url,
    ) {
        Ok(response) => {
            println!("Delete entry successful: {}", response.status);

            let data_vault = match app_state.vault.as_mut() {
                Some(vault) => vault,
                None => {
                    panic!("Failed to get reference to data vault");
                }
            };

            let entries_vault = &mut data_vault.entries_vault;

            match content_type {
                "password" => {
                    entries_vault.passwords.retain(|entry| entry.name != name);
                }
                "note" => {
                    entries_vault.notes.retain(|entry| entry.name != name);
                }
                "card" => {
                    entries_vault.cards.retain(|entry| entry.name != name);
                }
                "totp" => {
                    entries_vault
                        .totp_entries
                        .retain(|entry| entry.name != name);
                }
                _ => {
                    panic!("Invalid content type");
                }
            }

            Ok(())
        }
        Err(e) => {
            println!("Delete entry failed: {}", e);

            Err(e.to_string())
        }
    }
}
