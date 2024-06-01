use crate::model::TOTPEntry;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn generate_totp_token(totp_entry: TOTPEntry) -> Result<String, String> {
    let secret = match Secret::Encoded(totp_entry.secret.clone()).to_bytes() {
        Ok(secret) => secret,
        Err(e) => return Err(format!("Failed to decode secret: {}", e)),
    };

    let algorithm = match totp_entry.algorithm.as_str() {
        "SHA1" => Algorithm::SHA1,
        "SHA256" => Algorithm::SHA256,
        "SHA512" => Algorithm::SHA512,
        _ => return Err("Invalid algorithm".to_string()),
    };

    let totp = match TOTP::new(
        algorithm,
        totp_entry.digits,
        totp_entry.skew,
        totp_entry.period,
        secret,
    ) {
        Ok(totp) => totp,
        Err(e) => return Err(format!("Failed to create TOTP instance: {}", e)),
    };

    match totp.generate_current() {
        Ok(token) => Ok(token),
        Err(e) => Err(format!("Failed to generate TOTP token: {}", e)),
    }
}
