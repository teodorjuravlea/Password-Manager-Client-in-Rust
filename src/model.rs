use aes_gcm_siv::Aes256GcmSiv;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::encryption::generate_all_master_ciphers;
use crate::entries::fill_data_vault_from_response;

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

#[derive(Debug, Serialize)]
pub struct AddEncryptedDataEntryRequest {
    pub name: String,
    pub content: Vec<u8>,
    pub nonce: Vec<u8>,
    pub content_type: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateEncryptedDataEntryRequest {
    pub content_type: String,
    pub old_name: String,
    pub new_name: String,
    pub new_content: Vec<u8>,
    pub new_nonce: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct DeleteEncryptedDataEntryRequest {
    pub name: String,
    pub content_type: String,
}

// Response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilteredUser {
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub status: String,
    pub data: FilteredUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleResponse {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedDataEntryResponse {
    pub status: String,
    pub data: EncryptedDataEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllEncryptedDataEntriesResponse {
    pub status: String,
    pub data: Vec<EncryptedDataEntry>,
}

// Data structures
#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
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
pub struct TOTPEntry {
    pub name: String,
    pub algorithm: String,
    pub secret: String,
    pub digits: usize,
    pub skew: u8,
    pub period: u64,
    pub created_at: String,
}

pub struct Ciphers {
    pub password_cipher: Aes256GcmSiv,
    pub note_cipher: Aes256GcmSiv,
    pub card_cipher: Aes256GcmSiv,
    pub totp_entry_cipher: Aes256GcmSiv,
}

pub struct EntriesVault {
    pub passwords: Vec<Password>,
    pub notes: Vec<Note>,
    pub cards: Vec<Card>,
    pub totp_entries: Vec<TOTPEntry>,
}

pub struct DataVault {
    pub ciphers: Ciphers,
    pub entries_vault: EntriesVault,
}

impl DataVault {
    pub fn new(email: &str, password: &str) -> Result<DataVault, String> {
        Ok(DataVault {
            ciphers: match generate_all_master_ciphers(email, password) {
                Ok(ciphers) => ciphers,
                Err(e) => return Err(e),
            },
            entries_vault: EntriesVault {
                passwords: Vec::new(),
                notes: Vec::new(),
                cards: Vec::new(),
                totp_entries: Vec::new(),
            },
        })
    }

    pub fn fill(&mut self, encrypted_data_entries_response: GetAllEncryptedDataEntriesResponse) {
        fill_data_vault_from_response(self, encrypted_data_entries_response);
    }
}

// Encrypted data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedDataEntry {
    pub name: String,
    pub content: Vec<u8>,
    pub nonce: Vec<u8>,
    pub content_type: String,
}
