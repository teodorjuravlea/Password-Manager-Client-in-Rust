use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, KeySizeUser, Nonce,
};
use argon2::{
    password_hash::{Encoding, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn encrypt_data_entry(
    data_entry_string: &str,
    key: GenericArray<u8, <Aes256GcmSiv as KeySizeUser>::KeySize>,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let cipher = Aes256GcmSiv::new(&key);
    let nonce = Nonce::from_slice(b"unique nonce");

    match cipher.encrypt(nonce, data_entry_string.as_bytes()) {
        Ok(ciphertext) => Ok((ciphertext, nonce.to_vec())),
        Err(_) => Err("Failed to encrypt data entry".to_string()),
    }
}

pub fn decrypt_data_entry(
    ciphertext: Vec<u8>,
    key: GenericArray<u8, <Aes256GcmSiv as KeySizeUser>::KeySize>,
    nonce: Vec<u8>,
) -> Result<String, String> {
    let cipher = Aes256GcmSiv::new(&key);

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
pub fn generate_master_key(email: &str, password: &str, data_entry_type: &str) -> String {
    let unhashed_master_key = format!("{}{}{}", email, password, data_entry_type);

    let deterministic_salt = generate_deterministic_salt(&unhashed_master_key).unwrap();

    let master_key = match Argon2::default()
        .hash_password(unhashed_master_key.as_bytes(), deterministic_salt.as_salt())
    {
        Ok(hash) => hash.to_string(),
        Err(_) => panic!("Failed to hash master key"),
    };

    master_key
}

pub fn generate_deterministic_salt(password: &str) -> Result<SaltString, String> {
    let mut password_b64 = Vec::new();

    match Encoding::B64.encode(password.as_bytes(), &mut password_b64) {
        Ok(password_b64_slice) => match SaltString::from_b64(password_b64_slice) {
            Ok(salt) => Ok(salt),
            Err(_) => Err("Failed to generate salt".to_string()),
        },

        Err(_) => Err("Failed to encode password".to_string()),
    }
}
