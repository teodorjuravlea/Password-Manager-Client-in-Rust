use crate::encryption;
use crate::model;

pub fn test_encryption() {
    let email = "lmao@example.com";
    let password = "veryepicpassword1738";

    let saved_password = model::Password {
        name: "My Password".to_string(),
        username: "myusername".to_string(),
        password: "savedpassword1738".to_string(),
        url: "https://example.com".to_string(),
        expiration_date: "not dodays date".to_string(),
        created_at: "dodays date".to_string(),
    };

    let master_cipher = match encryption::generate_master_cipher(email, password, "password") {
        Ok(cipher) => cipher,
        Err(err) => panic!("{}", err),
    };

    let saved_password_string = serde_json::to_string(&saved_password).unwrap();

    let encrypted_password =
        match encryption::encrypt_data_entry(saved_password_string.as_str(), &master_cipher) {
            Ok(encrypted_password) => encrypted_password,
            Err(err) => panic!("{}", err),
        };

    println!(
        "Encrypted password: {:#?}, nonce: {:#?}",
        encrypted_password.0, encrypted_password.1
    );

    let decrypted_password = match encryption::decrypt_data_entry(
        encrypted_password.0,
        &master_cipher,
        encrypted_password.1,
    ) {
        Ok(decrypted_password) => decrypted_password,
        Err(err) => panic!("{}", err),
    };

    println!("Decrypted password: {:?}", decrypted_password);
}
