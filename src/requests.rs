use crate::model::{LoginRequest, RegisterRequest, UserResponse};

pub fn login_request(
    email: &str,
    password: &str,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<String, String> {
    let request = LoginRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => {
            let login_response = response.text().unwrap();
            Ok(login_response)
        }
        Err(_) => Err("Login failed".to_string()),
    }
}

pub fn register_request(
    email: &str,
    password: &str,
    password2: &str,
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<String, String> {
    if password != password2 {
        return Err("Passwords do not match".to_string());
    }

    let request = RegisterRequest {
        email: email.to_string(),
        password: password.to_string(),
    };

    let response = client.post(url).json(&request).send();

    match response {
        Ok(response) => {
            let register_response = response.text().unwrap();
            Ok(register_response)
        }
        Err(_) => Err("Register failed".to_string()),
    }
}
