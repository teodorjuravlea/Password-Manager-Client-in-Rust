use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};

use crate::model::{Ciphers, EncryptedDataEntry};

pub fn encrypt_data_entry(
    data_entry_string: &str,
    cipher: &Aes256GcmSiv,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let nonce = Nonce::from_slice(b"unique nonce");

    match cipher.encrypt(nonce, data_entry_string.as_bytes()) {
        Ok(ciphertext) => Ok((ciphertext, nonce.to_vec())),
        Err(e) => Err(format!("Failed to encrypt data entry: {}", e)),
    }
}

pub fn decrypt_data_entry(
    data_entry: EncryptedDataEntry,
    cipher: &Aes256GcmSiv,
) -> Result<String, String> {
    let ciphertext = data_entry.content;
    let nonce = Nonce::from_slice(data_entry.nonce.as_slice());

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(decrypted) => match String::from_utf8(decrypted) {
            Ok(decrypted_string) => Ok(decrypted_string),
            Err(e) => Err(format!("Failed to decode string: {}", e)),
        },
        Err(e) => Err(format!("Failed to decrypt data entry: {}", e)),
    }
}

pub fn generate_all_master_ciphers(email: &str, password: &str) -> Result<Ciphers, String> {
    let password_cipher = match generate_master_cipher(email, password, "password") {
        Ok(cipher) => cipher,
        Err(e) => return Err(e),
    };

    let note_cipher = match generate_master_cipher(email, password, "note") {
        Ok(cipher) => cipher,
        Err(e) => return Err(e),
    };

    let card_cipher = match generate_master_cipher(email, password, "card") {
        Ok(cipher) => cipher,
        Err(e) => return Err(e),
    };

    let totp_entry_cipher = match generate_master_cipher(email, password, "totp_entry") {
        Ok(cipher) => cipher,
        Err(e) => return Err(e),
    };

    Ok(Ciphers {
        password_cipher,
        note_cipher,
        card_cipher,
        totp_entry_cipher,
    })
}
pub fn generate_master_cipher(
    email: &str,
    password: &str,
    data_entry_type: &str,
) -> Result<Aes256GcmSiv, String> {
    let unhashed_master_key = generate_deterministic_key(email, password, data_entry_type);

    let deterministic_salt = match generate_deterministic_salt(&unhashed_master_key) {
        Ok(salt) => salt,
        Err(e) => return Err(e),
    };

    let master_key_hash = match Argon2::default()
        .hash_password(unhashed_master_key.as_bytes(), deterministic_salt.as_salt())
    {
        Ok(hash) => hash.hash,
        Err(e) => return Err(format!("Failed to hash password: {}", e)),
    };

    let master_key_output = match master_key_hash {
        Some(hash) => hash,
        None => return Err("Failed to generate master key".to_string()),
    };

    let master_key_string = master_key_output.as_bytes();

    let master_key = GenericArray::from_slice(master_key_string);

    Ok(Aes256GcmSiv::new(master_key))
}

pub fn generate_deterministic_key(email: &str, password: &str, data_entry_type: &str) -> String {
    format!("{}{}{}", email, password, data_entry_type)
}

pub fn generate_deterministic_salt(password: &str) -> Result<SaltString, String> {
    match SaltString::encode_b64(password.as_bytes()) {
        Ok(salt) => Ok(salt),
        Err(e) => Err(format!("Failed to generate salt: {}", e)),
    }
}
