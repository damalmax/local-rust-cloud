use std::io::{Error, ErrorKind};

use crate::random::{generate_char_sequence, ALPHANUMERIC_CAPITALIZED_CHARSET};

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
