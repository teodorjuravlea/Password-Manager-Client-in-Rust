use crate::model::{
    AddEncryptedDataEntryRequest, DeleteEncryptedDataEntryRequest, EncryptedDataEntry,
    EncryptedDataEntryResponse, GetAllEncryptedDataEntriesResponse, LoginRequest, RegisterRequest,
    ServerResponse, UpdateEncryptedDataEntryRequest, UserResponse,
};

pub fn login_request(
    email: &str,
    password: &str,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<ServerResponse, String> {
    let request = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => match response.json::<ServerResponse>() {
            Ok(login_response) => Ok(login_response),
            Err(e) => Err(format!("Login failed: {}", e)),
        },
        Err(e) => Err(format!("Login failed: {}", e)),
    }
}

pub fn register_request(
    email: &str,
    password: &str,
    password2: &str,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<UserResponse, String> {
    if password != password2 {
        return Err("Passwords do not match".to_string());
    }

    let request = RegisterRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => match response.json::<UserResponse>() {
            Ok(register_response) => Ok(register_response),
            Err(e) => Err(format!("Register failed: {}", e)),
        },
        Err(e) => Err(format!("Register failed: {}", e)),
    }
}

pub fn logout_request(client: &reqwest::blocking::Client, url: &str) -> Result<String, String> {
    let response = client.get(url).send();

    match response {
        Ok(response) => match response.text() {
            Ok(logout_response) => Ok(logout_response),
            Err(e) => Err(format!("Logout failed: {}", e)),
        },
        Err(e) => Err(format!("Logout failed: {}", e)),
    }
}

pub fn add_encrypted_data_entry_request(
    data_entry: EncryptedDataEntry,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<EncryptedDataEntryResponse, String> {
    let request = AddEncryptedDataEntryRequest {
        name: data_entry.name.clone(),
        content: data_entry.content.clone(),
        nonce: data_entry.nonce.clone(),
        content_type: data_entry.content_type.clone(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => match response.json::<EncryptedDataEntryResponse>() {
            Ok(add_response) => Ok(add_response),
            Err(e) => Err(format!("Add data entry failed: {}", e)),
        },
        Err(e) => Err(format!("Add data entry failed: {}", e)),
    }
}

pub fn update_encrypted_data_entry_request(
    old_name: &str,
    new_name: &str,
    new_data_entry: EncryptedDataEntry,
    content_type: &str,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<EncryptedDataEntryResponse, String> {
    let request = UpdateEncryptedDataEntryRequest {
        old_name: old_name.to_string(),
        new_name: new_name.to_string(),
        new_content: new_data_entry.content.clone(),
        new_nonce: new_data_entry.nonce.clone(),
        content_type: content_type.to_string(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => match response.json::<EncryptedDataEntryResponse>() {
            Ok(update_response) => Ok(update_response),
            Err(e) => Err(format!("Update data entry failed: {}", e)),
        },
        Err(e) => Err(format!("Update data entry failed: {}", e)),
    }
}

pub fn delete_encrypted_data_entry_request(
    name: &str,
    content_type: &str,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<String, String> {
    let request = DeleteEncryptedDataEntryRequest {
        name: name.to_string(),
        content_type: content_type.to_string(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => match response.text() {
            Ok(delete_response) => Ok(delete_response),
            Err(e) => Err(format!("Delete data entry failed: {}", e)),
        },
        Err(e) => Err(format!("Delete data entry failed: {}", e)),
    }
}

pub fn get_all_encrypted_data_entries_request(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<GetAllEncryptedDataEntriesResponse, String> {
    let response = client.get(url).send();

    match response {
        Ok(response) => match response.json::<GetAllEncryptedDataEntriesResponse>() {
            Ok(data_entries) => Ok(data_entries),
            Err(e) => Err(format!("Get all data entries failed: {}", e)),
        },
        Err(e) => Err(format!("Get all data entries failed: {}", e)),
    }
}
