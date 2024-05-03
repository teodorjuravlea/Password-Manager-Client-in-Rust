use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// Request structures
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// Response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct FilteredUser {
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub data: UserData,
    pub status: String,
}

// Data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub password: String,
    pub expiration_date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub name: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub cardholder_name: String,
    pub card_number: String,
    pub security_code: String,
    pub expiration_date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtpToken {
    pub name: String,
    pub token: String,
}

// Encrypted data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedPassword {
    pub name: String,
    pub password: String,
    pub expiration_date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedNote {
    pub name: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedCard {
    pub name: String,
    pub cardholder_name: String,
    pub card_number: String,
    pub security_code: String,
    pub expiration_date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedOtpToken {
    pub name: String,
    pub token: String,
}
