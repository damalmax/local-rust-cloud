use rand::Rng;

#[allow(dead_code)]
const ALPHA_CAPITALIZED_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const ALPHA_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const ACCESS_KEY_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
const SECRET_ACCESS_KEY_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890/";
const SESSION_TOKEN_CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+1234567890/";

pub fn generate_token(charset: &[u8], size: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn generate_access_key() -> String {
    "ASIA".to_owned() + generate_token(ACCESS_KEY_CHARSET, 16).as_str()
}

pub fn generate_secret_access_key() -> String {
    generate_token(ALPHA_CHARSET, 10) + generate_token(SECRET_ACCESS_KEY_CHARSET, 30).as_str()
}

pub fn generate_session_token() -> String {
    generate_token(ALPHA_CHARSET, 20) + generate_token(SESSION_TOKEN_CHARSET, 940).as_str()
}
