use aws_credential_types::provider::ProvideCredentials;

use self::test_suite::TestSuite;

mod test_suite;

#[cfg(test)]
mod assume_role;

const TEST_SUITE: TestSuite = TestSuite::new();

pub fn credentials_provider() -> impl ProvideCredentials {
    aws_credential_types::Credentials::new("access_key_id", "secret_access_key", Option::None, Option::None, "provider_name")
}
