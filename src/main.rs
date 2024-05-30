use model::DataVault;
use reqwest::blocking::Client;

pub mod encryption;
pub mod entries;
pub mod gui;
pub mod model;
pub mod requests;
pub mod tests;

struct AppState {
    client: Client,
    url: String,
    vault: DataVault,
}

fn main() {
    //tests::test_encryption::test_encryption();
    //tests::test_enc_req::test_enc_req();
    tests::test_gui::test_gui();
}
