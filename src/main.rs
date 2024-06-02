use entries::fill_data_vault_from_response;
use model::DataVault;
use requests::get_all_encrypted_data_entries_request;
use reqwest::blocking::Client;
use std::rc::Rc;

pub mod constraints;
pub mod encryption;
pub mod entries;
pub mod gui;
pub mod model;
pub mod requests;
pub mod tests;
pub mod totp;

pub struct AppState {
    client: Client,
    base_url: String,

    vault: Option<DataVault>,
    is_logged_in: bool,
}

fn main() {
    //tests::test_encryption::test_encryption();
    //tests::test_enc_req::test_enc_req();

    // Create a reqwest client with a cookie store
    let reqwest_client = match reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
    {
        Ok(client) => client,
        Err(e) => {
            println!("Failed to create reqwest client: {}", e);
            return;
        }
    };

    // Initialize the app state as a shared resource
    let mut state = Rc::new(AppState {
        client: reqwest_client,
        base_url: "http://localhost:8080".to_string(),
        vault: None,
        is_logged_in: false,
    });

    //tests::test_gui::test_gui(state.clone());

    // Run the login prompt
    gui::authentication::run_login_prompt(state.clone());

    if !state.is_logged_in {
        return;
    }

    let encrypted_entries_response =
        match get_all_encrypted_data_entries_request(&state.client, &state.base_url) {
            Ok(encrypted_entries) => encrypted_entries,
            Err(e) => {
                println!("Failed to get encrypted entries: {}", e);
                return;
            }
        };

    // Fill the data vault
    match Rc::get_mut(&mut state) {
        Some(app_state) => match app_state.vault.as_mut() {
            Some(vault) => {
                fill_data_vault_from_response(vault, encrypted_entries_response);
            }
            None => {
                println!("Failed to get mutable reference to data vault");
                return;
            }
        },
        None => {
            println!("Failed to get mutable reference to app state");
            return;
        }
    }
    //tests::test_totp::test_totp();
}
