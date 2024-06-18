use entries::fill_data_vault_from_response;
use model::DataVault;
use requests::get_all_encrypted_data_entries_request;
use reqwest::blocking::Client;
use std::{cell::RefCell, rc::Rc};

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
    let state = Rc::new(RefCell::new(AppState {
        client: reqwest_client,
        base_url: "http://localhost:8080".to_string(),
        vault: None,
        is_logged_in: false,
    }));

    // Run the auth prompt
    //gui::auth_prompt::run_auth_prompt(state.clone());

    // Manual auth
    {
        match requests::register_request(
            "lmao@example.com",
            "passwordxddd",
            "passwordxddd",
            &state.borrow().client,
            &state.borrow().base_url,
        ) {
            Ok(response) => println!("Register response: {:?}", response),
            Err(e) => println!("Register failed: {}", e),
        };

        // Login
        match requests::login_request(
            "lmao@example.com",
            "passwordxddd",
            &state.borrow().client,
            &state.borrow().base_url,
        ) {
            Ok(response) => println!("Login response: {:?}", response),
            Err(e) => println!("Login failed: {}", e),
        };

        state.borrow_mut().is_logged_in = true;

        let data_vault = match DataVault::new("lmao@example.com", "passwordxddd") {
            Ok(data_vault) => data_vault,
            Err(e) => {
                panic!("Error creating data vault: {}", e);
            }
        };

        state.borrow_mut().vault = Some(data_vault);
    }

    if !state.borrow().is_logged_in {
        return;
    }

    let encrypted_entries_response = match get_all_encrypted_data_entries_request(
        &state.borrow().client,
        &state.borrow().base_url,
    ) {
        Ok(encrypted_entries) => encrypted_entries,
        Err(e) => {
            println!("Failed to get encrypted entries: {}", e);
            return;
        }
    };

    // Fill the data vault
    match state.borrow_mut().vault.as_mut() {
        Some(vault) => {
            fill_data_vault_from_response(vault, encrypted_entries_response);
        }
        None => {
            println!("Failed to get mutable reference to data vault");
            return;
        }
    };

    // Run main window
    gui::main_window::run_main_window(state.clone());
}
