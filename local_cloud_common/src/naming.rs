use std::io::{Error, ErrorKind};

use rand::Rng;

const ALPHANUMERIC_CAPITALIZED_CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn generate_char_sequence(charset: &[u8], length: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}

pub fn generate_id(prefix: &str, length: usize) -> Result<String, Error> {
    let prefix_length = prefix.chars().count();
    if prefix_length > length {
        Err(Error::new(ErrorKind::InvalidData, "Failed generate new unique identifier."))
    } else {
        Ok(prefix.to_string()
            + generate_char_sequence(ALPHANUMERIC_CAPITALIZED_CHARSET, length - prefix_length).as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id_for_managed_policy() {
        let result = generate_id("ANPA", 21);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.starts_with("ANPA"));
        assert_eq!(result.chars().count(), 21);
    }
}
