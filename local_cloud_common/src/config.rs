/// Retrieves environment variable string value for the given key. If the environment variable is not set, default value will be used.
pub fn get_string_env_with_default(env_key: &str, default_value: &str) -> impl Into<String> {
    std::env::var(env_key).unwrap_or_else(|_| {
        log::warn!("{} env variable is not set. Using default value: {}", env_key, default_value);
        default_value.into()
    })
}

/// Retrieves environment variable bool value for the given key. If the environment variable is not set, default value will be used.
/// If the value is not a valid boolean, `false` value will be used by default.
pub fn get_bool_env_with_default(env_key: &str, default_value: bool) -> impl Into<bool> {
    std::env::var(env_key)
        .map(|v| String::from("true").eq(&v))
        .unwrap_or_else(|_| {
            log::warn!("{} env variable is not set. Using default value: {}", env_key, default_value);
            default_value
        })
}

/// Retrieves environment variable u16 value for the given key. If the environment variable is not set, default value will be used.
pub fn get_u16_env_with_default(env_key: &str, default_value: u16) -> u16 {
    std::env::var(env_key)
        .map(|v| {
            v.parse::<u16>().unwrap_or_else(|_| {
                log::warn!("failed to parse {} env variable. Using default value: {}", env_key, default_value);
                default_value
            })
        })
        .unwrap_or_else(|_| {
            log::warn!("{} env variable is not set. Using default value: {}", env_key, default_value);
            default_value
        })
}
