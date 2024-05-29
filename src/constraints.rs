// Authentication
pub fn is_password_valid(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 64
}

pub fn is_email_valid(email: &str) -> bool {
    let re = match regex::Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    ) {
        Ok(re) => re,
        Err(_) => return false,
    };

    re.is_match(email)
}
