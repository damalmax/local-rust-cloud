use argon2::Argon2;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};

use crate::http::aws::iam::actions::error::ApiErrorKind;
use crate::http::aws::iam::operations::error::ActionError;

pub(crate) fn password_hash(password: impl AsRef<[u8]>) -> Result<String, ActionError> {
    let salt = SaltString::generate(OsRng);

    Argon2::default()
        .hash_password(password.as_ref(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| {
            ActionError::new(ApiErrorKind::ServiceFailure, "Password hashing error. Please contact support team.")
        })
}
