use rand::Rng;

pub const ALPHANUMERIC_CAPITALIZED_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHANUMERIC_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn generate_char_sequence(charset: &[u8], length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn generate_bytes_slice(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    (0..length).map(|_| rng.gen()).collect()
}
