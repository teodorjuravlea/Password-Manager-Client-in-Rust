use crate::encryption::{decrypt_data_entry, encrypt_data_entry};
use crate::model::{
    Card, Ciphers, DataVault, EncryptedDataEntry, EntriesVault, GetAllEncryptedDataEntriesResponse,
    Note, Password, TOTPEntry,
};
use aes_gcm_siv::Aes256GcmSiv;

// Create entry functions
pub fn create_password_entry(
    name: &str,
    username: &str,
    password: &str,
    url: &str,
    expiration_date: &str,
) -> Password {
    Password {
        name: name.to_string(),
        username: username.to_string(),
        password: password.to_string(),
        url: url.to_string(),
        expiration_date: expiration_date.to_string(),
        created_at: "".to_string(),
    }
}

pub fn create_note_entry(name: &str, content: &str) -> Note {
    Note {
        name: name.to_string(),
        content: content.to_string(),
        created_at: "".to_string(),
    }
}

pub fn create_card_entry(
    name: &str,
    cardholder_name: &str,
    card_number: &str,
    security_code: &str,
    expiration_date: &str,
) -> Card {
    Card {
        name: name.to_string(),
        cardholder_name: cardholder_name.to_string(),
        card_number: card_number.to_string(),
        security_code: security_code.to_string(),
        expiration_date: expiration_date.to_string(),
        created_at: "".to_string(),
    }
}

// Encrypt entry functions
pub fn encrypt_password_entry(
    password: &Password,
    cipher: &Aes256GcmSiv,
) -> Result<EncryptedDataEntry, String> {
    let serialized_data = match serde_json::to_string(&password) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to serialize password: {}", e)),
    };

    match encrypt_data_entry(&serialized_data, cipher) {
        Ok((content, nonce)) => Ok(EncryptedDataEntry {
            name: password.name.clone(),
            content,
            nonce,
            content_type: "password".to_string(),
        }),
        Err(e) => Err(e),
    }
}

pub fn encrypt_note_entry(
    note: &Note,
    cipher: &Aes256GcmSiv,
) -> Result<EncryptedDataEntry, String> {
    let serialized_data = match serde_json::to_string(&note) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to serialize note: {}", e)),
    };

    match encrypt_data_entry(&serialized_data, cipher) {
        Ok((content, nonce)) => Ok(EncryptedDataEntry {
            name: note.name.clone(),
            content,
            nonce,
            content_type: "note".to_string(),
        }),
        Err(e) => Err(e),
    }
}

pub fn encrypt_card_entry(
    card: &Card,
    cipher: &Aes256GcmSiv,
) -> Result<EncryptedDataEntry, String> {
    let serialized_data = match serde_json::to_string(&card) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to serialize card: {}", e)),
    };

    match encrypt_data_entry(&serialized_data, cipher) {
        Ok((content, nonce)) => Ok(EncryptedDataEntry {
            name: card.name.clone(),
            content,
            nonce,
            content_type: "card".to_string(),
        }),
        Err(e) => Err(e),
    }
}

pub fn encrypt_totp_entry(
    totp_entry: &TOTPEntry,
    cipher: &Aes256GcmSiv,
) -> Result<EncryptedDataEntry, String> {
    let serialized_data = match serde_json::to_string(&totp_entry) {
        Ok(data) => data,
        Err(e) => return Err(format!("Failed to serialize TOTP entry: {}", e)),
    };

    match encrypt_data_entry(&serialized_data, cipher) {
        Ok((content, nonce)) => Ok(EncryptedDataEntry {
            name: totp_entry.name.clone(),
            content,
            nonce,
            content_type: "totp_entry".to_string(),
        }),
        Err(e) => Err(e),
    }
}

// Decrypt entry functions
pub fn decrypt_password_entry(
    encrypted_data_entry: EncryptedDataEntry,
    cipher: &Aes256GcmSiv,
) -> Result<Password, String> {
    match decrypt_data_entry(encrypted_data_entry, cipher) {
        Ok(data) => match serde_json::from_str::<Password>(&data) {
            Ok(password) => Ok(password),
            Err(e) => Err(format!("Failed to deserialize password: {}", e)),
        },
        Err(e) => Err(e),
    }
}

pub fn decrypt_note_entry(
    encrypted_data_entry: EncryptedDataEntry,
    cipher: &Aes256GcmSiv,
) -> Result<Note, String> {
    match decrypt_data_entry(encrypted_data_entry, cipher) {
        Ok(data) => match serde_json::from_str::<Note>(&data) {
            Ok(note) => Ok(note),
            Err(e) => Err(format!("Failed to deserialize note: {}", e)),
        },
        Err(e) => Err(e),
    }
}

pub fn decrypt_card_entry(
    encrypted_data_entry: EncryptedDataEntry,
    cipher: &Aes256GcmSiv,
) -> Result<Card, String> {
    match decrypt_data_entry(encrypted_data_entry, cipher) {
        Ok(data) => match serde_json::from_str::<Card>(&data) {
            Ok(card) => Ok(card),
            Err(e) => Err(format!("Failed to deserialize card: {}", e)),
        },
        Err(e) => Err(e),
    }
}

pub fn decrypt_totp_entry(
    encrypted_data_entry: EncryptedDataEntry,
    cipher: &Aes256GcmSiv,
) -> Result<TOTPEntry, String> {
    match decrypt_data_entry(encrypted_data_entry, cipher) {
        Ok(data) => match serde_json::from_str::<TOTPEntry>(&data) {
            Ok(totp_entry) => Ok(totp_entry),
            Err(e) => Err(format!("Failed to deserialize TOTP entry: {}", e)),
        },
        Err(e) => Err(e),
    }
}

// Vault functions
pub fn fill_data_vault_from_response(
    data_vault: &mut DataVault,
    response: GetAllEncryptedDataEntriesResponse,
) {
    for encrypted_data_entry in response.data {
        match encrypted_data_entry.content_type.as_str() {
            "password" => {
                match decrypt_password_entry(
                    encrypted_data_entry,
                    &data_vault.ciphers.password_cipher,
                ) {
                    Ok(password) => data_vault.entries_vault.passwords.push(password),
                    Err(e) => println!("{}", e),
                }
            }
            "note" => {
                match decrypt_note_entry(encrypted_data_entry, &data_vault.ciphers.note_cipher) {
                    Ok(note) => data_vault.entries_vault.notes.push(note),
                    Err(e) => println!("{}", e),
                }
            }
            "card" => {
                match decrypt_card_entry(encrypted_data_entry, &data_vault.ciphers.card_cipher) {
                    Ok(card) => data_vault.entries_vault.cards.push(card),
                    Err(e) => println!("{}", e),
                }
            }
            "totp_entry" => {
                match decrypt_totp_entry(
                    encrypted_data_entry,
                    &data_vault.ciphers.totp_entry_cipher,
                ) {
                    Ok(totp_entry) => data_vault.entries_vault.totp_entries.push(totp_entry),
                    Err(e) => println!("{}", e),
                }
            }
            _ => println!(
                "Unknown content type: {}",
                encrypted_data_entry.content_type
            ),
        }
    }
}

pub fn encrypt_entry_vault(
    entry_vault: &EntriesVault,
    ciphers: Ciphers,
) -> Vec<EncryptedDataEntry> {
    let mut encrypted_entries = Vec::new();

    for password in &entry_vault.passwords {
        match encrypt_password_entry(password, &ciphers.password_cipher) {
            Ok(encrypted_entry) => encrypted_entries.push(encrypted_entry),
            Err(e) => println!("{}", e),
        }
    }
    for note in &entry_vault.notes {
        match encrypt_note_entry(note, &ciphers.note_cipher) {
            Ok(encrypted_entry) => encrypted_entries.push(encrypted_entry),
            Err(e) => println!("{}", e),
        }
    }
    for card in &entry_vault.cards {
        match encrypt_card_entry(card, &ciphers.card_cipher) {
            Ok(encrypted_entry) => encrypted_entries.push(encrypted_entry),
            Err(e) => println!("{}", e),
        }
    }
    for totp_entry in &entry_vault.totp_entries {
        match encrypt_totp_entry(totp_entry, &ciphers.totp_entry_cipher) {
            Ok(encrypted_entry) => encrypted_entries.push(encrypted_entry),
            Err(e) => println!("{}", e),
        }
    }

    encrypted_entries
}
