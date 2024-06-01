use crate::constraints;
use crate::model::{
    AddEncryptedDataEntryRequest, DeleteEncryptedDataEntryRequest, EncryptedDataEntry,
    EncryptedDataEntryResponse, ErrorResponse, GetAllEncryptedDataEntriesResponse, LoginRequest,
    RegisterRequest, SimpleResponse, UpdateEncryptedDataEntryRequest, UserResponse,
};

pub fn login_request(
    email: &str,
    password: &str,
    client: &reqwest::blocking::Client,
    base_url: &str,
) -> Result<SimpleResponse, String> {
    if (!constraints::is_email_valid(email)) || (!constraints::is_password_valid(password)) {
        return Err("Invalid email or password".to_string());
    }

    let request = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let full_url = format!("{}/login", base_url);

    let response = client.post(full_url).json(&request).send();

    match response {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json_response) => {
                let json_response_copy = json_response.clone();

                match serde_json::from_value::<SimpleResponse>(json_response_copy) {
                    Ok(login_response) => Ok(login_response),
                    Err(_) => match serde_json::from_value::<ErrorResponse>(json_response) {
                        Ok(error_response) => Err(error_response.message),
                        Err(_) => Err("Error parsing response".to_string()),
                    },
                }
            }
            Err(_) => Err("Error parsing response".to_string()),
        },
        Err(_) => Err("Error sending request".to_string()),
    }
}

pub fn register_request(
    email: &str,
    password: &str,
    password2: &str,
    client: &reqwest::blocking::Client,
    base_url: &str,
) -> Result<UserResponse, String> {
    if password != password2 {
        return Err("Passwords do not match".to_string());
    }

    if (!constraints::is_email_valid(email)) || (!constraints::is_password_valid(password)) {
        return Err("Invalid email or password".to_string());
    }

    let request = RegisterRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let full_url = format!("{}/register", base_url);

    let response = client.post(full_url).json(&request).send();

    match response {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json_response) => {
                let json_response_copy = json_response.clone();

                match serde_json::from_value::<UserResponse>(json_response_copy) {
                    Ok(register_response) => Ok(register_response),
                    Err(_) => match serde_json::from_value::<ErrorResponse>(json_response) {
                        Ok(error_response) => Err(error_response.message),
                        Err(_) => Err("Error parsing response".to_string()),
                    },
                }
            }
            Err(_) => Err("Error parsing response".to_string()),
        },
        Err(_) => Err("Error sending request".to_string()),
    }
}

pub fn logout_request(
    client: &reqwest::blocking::Client,
    base_url: &str,
) -> Result<String, String> {
    let full_url = format!("{}/logout", base_url);

    let response = client.get(full_url).send();

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
    base_url: &str,
) -> Result<EncryptedDataEntryResponse, String> {
    let request = AddEncryptedDataEntryRequest {
        name: data_entry.name.clone(),
        content: data_entry.content.clone(),
        nonce: data_entry.nonce.clone(),
        content_type: data_entry.content_type.clone(),
    };

    let full_url = format!("{}/add_encrypted_data_entry", base_url);

    let response = client.post(full_url).json(&request).send();

    match response {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json_response) => {
                let json_response_copy = json_response.clone();

                match serde_json::from_value::<EncryptedDataEntryResponse>(json_response_copy) {
                    Ok(add_response) => Ok(add_response),
                    Err(_) => match serde_json::from_value::<ErrorResponse>(json_response) {
                        Ok(error_response) => Err(error_response.message),
                        Err(_) => Err("Error parsing response".to_string()),
                    },
                }
            }
            Err(_) => Err("Error parsing response".to_string()),
        },
        Err(_) => Err("Error sending request".to_string()),
    }
}

pub fn update_encrypted_data_entry_request(
    old_name: &str,
    new_name: &str,
    new_data_entry: EncryptedDataEntry,
    content_type: &str,
    client: &reqwest::blocking::Client,
    base_url: &str,
) -> Result<EncryptedDataEntryResponse, String> {
    let request = UpdateEncryptedDataEntryRequest {
        old_name: old_name.to_string(),
        new_name: new_name.to_string(),
        new_content: new_data_entry.content.clone(),
        new_nonce: new_data_entry.nonce.clone(),
        content_type: content_type.to_string(),
    };

    let full_url = format!("{}/update_encrypted_data_entry", base_url);

    let response = client.post(full_url).json(&request).send();

    match response {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json_response) => {
                let json_response_copy = json_response.clone();

                match serde_json::from_value::<EncryptedDataEntryResponse>(json_response_copy) {
                    Ok(update_response) => Ok(update_response),
                    Err(_) => match serde_json::from_value::<ErrorResponse>(json_response) {
                        Ok(error_response) => Err(error_response.message),
                        Err(_) => Err("Error parsing response".to_string()),
                    },
                }
            }
            Err(_) => Err("Error parsing response".to_string()),
        },
        Err(_) => Err("Error sending request".to_string()),
    }
}

pub fn delete_encrypted_data_entry_request(
    name: &str,
    content_type: &str,
    client: &reqwest::blocking::Client,
    base_url: &str,
) -> Result<SimpleResponse, String> {
    let request = DeleteEncryptedDataEntryRequest {
        name: name.to_string(),
        content_type: content_type.to_string(),
    };

    let full_url = format!("{}/delete_encrypted_data_entry", base_url);

    let response = client.post(full_url).json(&request).send();

    match response {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json_response) => {
                let json_response_copy = json_response.clone();

                match serde_json::from_value::<SimpleResponse>(json_response_copy) {
                    Ok(delete_response) => Ok(delete_response),
                    Err(_) => match serde_json::from_value::<ErrorResponse>(json_response) {
                        Ok(error_response) => Err(error_response.message),
                        Err(_) => Err("Error parsing response".to_string()),
                    },
                }
            }
            Err(_) => Err("Error parsing response".to_string()),
        },
        Err(_) => Err("Error sending request".to_string()),
    }
}

pub fn get_all_encrypted_data_entries_request(
    client: &reqwest::blocking::Client,
    base_url: &str,
) -> Result<GetAllEncryptedDataEntriesResponse, String> {
    let full_url = format!("{}/get_all_encrypted_data_entries", base_url);

    let response = client.get(full_url).send();

    match response {
        Ok(response) => match response.json::<serde_json::Value>() {
            Ok(json_response) => {
                let json_response_copy = json_response.clone();

                match serde_json::from_value::<GetAllEncryptedDataEntriesResponse>(
                    json_response_copy,
                ) {
                    Ok(data_response) => Ok(data_response),
                    Err(_) => match serde_json::from_value::<ErrorResponse>(json_response) {
                        Ok(error_response) => Err(error_response.message),
                        Err(_) => Err("Error parsing response".to_string()),
                    },
                }
            }
            Err(_) => Err("Error parsing response".to_string()),
        },
        Err(_) => Err("Error sending request".to_string()),
    }
}
