use gui::app_top_wrapper::AppTopWrapper;
use model::DataVault;
use relm4::RelmApp;
use relm4_icons::initialize_icons;
use reqwest::blocking::Client;
use std::{cell::RefCell, rc::Rc};

pub mod constraints;
pub mod encryption;
pub mod entries;
pub mod gui;
pub mod model;
pub mod requests;
pub mod totp;

pub struct AppState {
    client: Client,
    base_url: String,

    vault: Option<DataVault>,
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
    }));

    initialize_icons();

    let app = RelmApp::new("password-manager-client");
    app.run::<AppTopWrapper>(state);
}
