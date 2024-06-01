use crate::{model::TOTPEntry, totp};

pub fn test_totp() {
    let totp_entry = TOTPEntry {
        name: "test".to_string(),
        algorithm: "SHA1".to_string(),
        secret: "S4PW7FLGZI3QRM6CPSVAFNADT46INJJ5".to_string(),
        digits: 6,
        skew: 0,
        period: 30,
        created_at: "dodays date".to_string(),
    };

    let token = totp::generate_totp_token(totp_entry).unwrap();
    println!("Generated TOTP token: {}", token);
}
