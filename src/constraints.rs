// Authentication
pub fn is_password_valid(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 64
}

pub fn is_email_valid(email: &str) -> bool {
    let re = match regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$") {
        Ok(re) => re,
        Err(_) => return false,
    };

    re.is_match(email)
}
