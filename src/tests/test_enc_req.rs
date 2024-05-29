use crate::encryption;
use crate::entries::{
    create_note_entry, create_password_entry, encrypt_note_entry, encrypt_password_entry,
};
use crate::model::{
    Card, Ciphers, EncryptedDataEntry, EncryptedDataEntryResponse, ErrorResponse, Note, OtpToken,
    Password, UserResponse,
};
use crate::requests;

pub fn test_print_all_data_entries(
    client: &reqwest::blocking::Client,
    url: &str,
    ciphers: &Ciphers,
) {
    let response = requests::get_all_encrypted_data_entries_request(
        &client,
        "http://localhost:8080/get_all_encrypted_data_entries",
    );

    let data_entries = response.unwrap().data;

    println!("Data entries:");
    for data_entry in data_entries {
        let unencrypted_content = encryption::decrypt_data_entry(
            data_entry.clone(),
            match data_entry.content_type.as_str() {
                "password" => &ciphers.password_cipher,
                "note" => &ciphers.note_cipher,
                "card" => &ciphers.card_cipher,
                "otp_token" => &ciphers.otp_token_cipher,
                _ => panic!("Invalid content type"),
            },
        )
        .unwrap();

        match data_entry.content_type.as_str() {
            "password" => {
                let password: Password = serde_json::from_str(&unencrypted_content).unwrap();
                println!("Password: {:?}", password);
            }
            "note" => {
                let note: Note = serde_json::from_str(&unencrypted_content).unwrap();
                println!("Note: {:?}", note);
            }
            "card" => {
                let card: Card = serde_json::from_str(&unencrypted_content).unwrap();
                println!("Card: {:?}", card);
            }
            "otp_token" => {
                let otp_token: OtpToken = serde_json::from_str(&unencrypted_content).unwrap();
                println!("Otp Token: {:?}", otp_token);
            }
            _ => panic!("Invalid content type"),
        }
    }
}

pub fn test_enc_req() {
    // Create a reqwest client with a cookie store
    let reqwest_client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();

    // Register a user
    match requests::register_request(
        "lmao@example.com",
        "passwordxddd",
        "passwordxddd",
        &reqwest_client,
        "http://localhost:8080/register",
    ) {
        Ok(response) => println!("Register response: {:?}", response),
        Err(e) => println!("Register failed: {}", e),
    };

    // Login
    match requests::login_request(
        "lmao@example.com",
        "passwordxddd",
        &reqwest_client,
        "http://localhost:8080/login",
    ) {
        Ok(response) => println!("Login response: {:?}", response),
        Err(e) => println!("Login failed: {}", e),
    };

    // Get user info
    match reqwest_client.get("http://localhost:8080/me").send() {
        Ok(response) => println!("User info response: {:?}", response.text().unwrap()),
        Err(e) => println!("User info failed: {}", e),
    }

    // Logout
    match requests::logout_request(&reqwest_client, "http://localhost:8080/logout") {
        Ok(response) => println!("Logout response: {:?}", response),
        Err(e) => println!("Logout failed: {}", e),
    };

    // Try to get user info again
    match reqwest_client.get("http://localhost:8080/me").send() {
        Ok(response) => println!("User info response: {:?}", response.text().unwrap()),
        Err(e) => println!("User info failed: {}", e),
    };

    // Login again
    match requests::login_request(
        "lmao@example.com",
        "passwordxddd",
        &reqwest_client,
        "http://localhost:8080/login",
    ) {
        Ok(response) => println!("Login response: {:?}", response.data),
        Err(e) => println!("Login failed: {}", e),
    };

    // Generate master ciphers
    let ciphers =
        encryption::generate_all_master_ciphers("lmao@example.com", "passwordxddd").unwrap();

    // Get user info
    match reqwest_client.get("http://localhost:8080/me").send() {
        Ok(response) => println!("User info response: {:?}", response.text().unwrap()),
        Err(e) => println!("User info failed: {}", e),
    }

    // Get all data entries
    test_print_all_data_entries(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
        &ciphers,
    );

    // Add a new encrypted password
    let password = create_password_entry(
        "My Password",
        "userfella",
        "newpassword123",
        "https://example.com",
        "sometime",
    );

    let data_entry = encrypt_password_entry(password, &ciphers.password_cipher).unwrap();

    match requests::add_encrypted_data_entry_request(
        data_entry,
        &reqwest_client,
        "http://localhost:8080/add_encrypted_data_entry",
    ) {
        Ok(response) => println!("Add encrypted data entry response: {:?}", response),
        Err(e) => println!("Add encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    test_print_all_data_entries(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
        &ciphers,
    );

    // Add another encrypted data entry
    let note = create_note_entry("My Note", "This is a note");

    let data_entry = encrypt_note_entry(note, &ciphers.note_cipher).unwrap();

    match requests::add_encrypted_data_entry_request(
        data_entry,
        &reqwest_client,
        "http://localhost:8080/add_encrypted_data_entry",
    ) {
        Ok(response) => println!("Add encrypted data entry response: {:?}", response),
        Err(e) => println!("Add encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    test_print_all_data_entries(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
        &ciphers,
    );

    // Update an encrypted data entry
    let password = create_password_entry(
        "Our Password",
        "userhomie",
        "changedpass246",
        "https://imample.com",
        "sometime",
    );

    let data_entry = encrypt_password_entry(password, &ciphers.password_cipher).unwrap();

    match requests::update_encrypted_data_entry_request(
        "My Password",
        "Our Password",
        data_entry,
        "password",
        &reqwest_client,
        "http://localhost:8080/update_encrypted_data_entry",
    ) {
        Ok(response) => println!("Update encrypted data entry response: {:?}", response),
        Err(e) => println!("Update encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    test_print_all_data_entries(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
        &ciphers,
    );

    // Delete an encrypted data entry
    match requests::delete_encrypted_data_entry_request(
        "My Note",
        "note",
        &reqwest_client,
        "http://localhost:8080/delete_encrypted_data_entry",
    ) {
        Ok(response) => println!("Delete encrypted data entry response: {:?}", response),
        Err(e) => println!("Delete encrypted data entry failed: {}", e),
    };

    // Get all data entries again
    test_print_all_data_entries(
        &reqwest_client,
        "http://localhost:8080/get_all_encrypted_data_entries",
        &ciphers,
    );
}
