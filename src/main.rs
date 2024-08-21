use entries::fill_data_vault_from_response;
use model::DataVault;
use relm4_icons::initialize_icons;
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

    initialize_icons();

    gui::app_top_wrapper::run_app(state.clone());
}
