use model::DataVault;
use reqwest::blocking::Client;

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
}

fn main() {
    //tests::test_encryption::test_encryption();
    tests::test_enc_req::test_enc_req();
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

    let state = AppState {
        client: reqwest_client,
        base_url: "http://localhost:8080".to_string(),
        vault: None,
    };

    //tests::test_gui::test_gui(state);
    //tests::test_totp::test_totp();
}
