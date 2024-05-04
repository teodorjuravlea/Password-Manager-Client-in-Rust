use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit, OsRng},
    Aes256GcmSiv, Key, KeySizeUser, Nonce,
};
use argon2::{
    password_hash::{Encoding, PasswordHash, PasswordHasher, PasswordVerifier, Salt, SaltString},
    Argon2,
};

pub fn encrypt_data_entry(
    data_entry_string: &str,
    cipher: Aes256GcmSiv,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let nonce = Nonce::from_slice(b"unique nonce");

    match cipher.encrypt(nonce, data_entry_string.as_bytes()) {
        Ok(ciphertext) => Ok((ciphertext, nonce.to_vec())),
        Err(_) => Err("Failed to encrypt data entry".to_string()),
    }
}

pub fn decrypt_data_entry(
    ciphertext: Vec<u8>,
    cipher: Aes256GcmSiv,
    nonce: Vec<u8>,
) -> Result<String, String> {
    let nonce = Nonce::from_slice(nonce.as_slice());

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(decrypted) => match String::from_utf8(decrypted) {
            Ok(decrypted_string) => Ok(decrypted_string),
            Err(_) => Err("Failed to decode string".to_string()),
        },
        Err(_) => Err("Failed to decrypt data entry".to_string()),
    }
}

// Generate 256-bit master key
pub fn generate_master_cipher(
    email: &str,
    password: &str,
    data_entry_type: &str,
) -> Result<Aes256GcmSiv, String> {
    let unhashed_master_key = format!("{}{}{}", email, password, data_entry_type);

    let deterministic_salt = generate_deterministic_salt(&unhashed_master_key).unwrap();

    let master_key_hash = match Argon2::default()
        .hash_password(unhashed_master_key.as_bytes(), deterministic_salt.as_salt())
    {
        Ok(hash) => hash.hash,
        Err(_) => panic!("Failed to hash master key"),
    };

    let master_key_output = master_key_hash.unwrap();
    let master_key_string = master_key_output.as_bytes();

    let master_key = GenericArray::from_slice(master_key_string);

    Ok(Aes256GcmSiv::new(master_key))
}

pub fn generate_deterministic_salt(password: &str) -> Result<SaltString, String> {
    match SaltString::encode_b64(password.as_bytes()) {
        Ok(salt) => Ok(salt),
        Err(_) => Err("Failed to encode password".to_string()),
    }
}
