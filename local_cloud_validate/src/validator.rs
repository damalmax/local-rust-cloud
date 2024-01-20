use std::fmt::Display;

use num_traits::Num;
use regex::Regex;

use crate::error::ValidationError;
use crate::ValidationErrorKind;

pub trait Validator {
    fn validate(&self) -> Result<(), ValidationError>;
}

pub trait NamedValidator {
    fn validate(&self, at: &str) -> Result<(), ValidationError>;
}

pub fn validate_named<T: NamedValidator>(value: Option<T>, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(item) => item.validate(at),
    }
}

pub fn validate_required<T>(value: Option<T>, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Err(ValidationError::new(
            ValidationErrorKind::Required,
            format!("Parameter '{at}' in missing in the request."),
        )),
        Some(_) => Ok(()),
    }
}

pub fn validate_chars(value: Option<&str>, valid_characters: &[char], at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(input) => {
            if input.chars().into_iter().all(|ch| valid_characters.contains(&ch)) {
                Ok(())
            } else {
                Err(ValidationError::new(
                    ValidationErrorKind::RegExp, // just another way to define RegExp validation
                    format!("'{at}' could contain the following characters only: '{:?}'.", valid_characters),
                ))
            }
        }
    }
}

pub fn validate_min<T>(value: Option<T>, min: T, at: &str) -> Result<(), ValidationError>
where
    T: Num + Ord + Display,
{
    match value {
        None => Ok(()),
        Some(num) => {
            if num < min {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("Parameter '{at}' value cannot be less than {min}."),
                ))
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_max<T>(value: Option<T>, max: T, at: &str) -> Result<(), ValidationError>
where
    T: Num + Ord + Display,
{
    match value {
        None => Ok(()),
        Some(num) => {
            if num > max {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("Parameter '{at}' value cannot be greater than {max}."),
                ))
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_array_size_min<T: Sized>(value: Option<&[T]>, min: usize, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(vec) => {
            if vec.len() < min {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("Number of '{at}' cannot be less than {min}."),
                ))
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_array_size_max<T: Sized>(value: Option<&[T]>, max: usize, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(vec) => {
            if vec.len() > max {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("Number of '{at}' cannot be greater than {max}."),
                ))
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_str_length_min(value: Option<&str>, min: usize, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(text) => {
            if text.chars().count() < min {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("Length of '{at}' value cannot be less than {min}."),
                ))
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_str_length_max(value: Option<&str>, max: usize, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(text) => {
            if text.chars().count() > max {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("Length of '{at}' value cannot be greater than {max}."),
                ))
            } else {
                Ok(())
            }
        }
    }
}

pub fn validate_regexp(value: Option<&str>, regex: &Regex, at: &str) -> Result<(), ValidationError> {
    match value {
        None => Ok(()),
        Some(text) => {
            if regex.is_match(text) {
                Ok(())
            } else {
                Err(ValidationError::new(
                    ValidationErrorKind::Min,
                    format!("'{at}' value must follow the following RegExp: '{}'.", regex.as_str()),
                ))
            }
        }
    }
}
