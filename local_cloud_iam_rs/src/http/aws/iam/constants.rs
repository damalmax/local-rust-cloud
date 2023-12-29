pub(crate) mod xml {
    pub const IAM_XMLNS: &str = "https://iam.amazonaws.com/doc/2010-05-08/";
}

pub(crate) mod account {
    pub const ACCOUNT_ALIAS_MIN_SIZE: usize = 3;
    pub const ACCOUNT_ALIAS_MAX_SIZE: usize = 63;
}

pub(crate) mod policy {
    use std::time::Duration;

    pub const INLINE_USER_POLICY_MAX_SIZE: usize = 2048;
    pub const INLINE_ROLE_POLICY_MAX_SIZE: usize = 10240;
    pub const INLINE_GROUP_POLICY_MAX_SIZE: usize = 5120;

    pub const MANAGED_POLICY_MAX_SIZE: usize = 6144;
    pub const MANAGED_POLICIES_PER_SESSION_MAX_COUNT: usize = 10;

    pub const POLICY_NAME_MAX_SIZE: usize = 128;
    pub const PATH_MAX_SIZE: usize = 512;
    pub const ROLE_NAME_MAX_SIZE: usize = 64;

    /// 15 minutes
    pub const ROLE_SESSION_DURATION_MIN_SEC: Duration = Duration::new(900, 0);
    /// 12 hours
    pub const ROLE_SESSION_DURATION_MAX_SEC: Duration = Duration::new(43200, 0);

    pub const ROLE_SESSION_DEFAULT_DURATION_SEC: Duration = Duration::new(3600, 0);
}

pub(crate) mod tag {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub static ref TAG_KEY_REGEX: Regex = Regex::new(r"^[\p{L}\p{Z}\p{N}_.:/=+\-@]+$").unwrap();
        pub static ref TAG_VALUE_REGEX: Regex = Regex::new(r"^[\p{L}\p{Z}\p{N}_.:/=+\-@]*$").unwrap();
    }

    pub const TAG_KEY_MIN_SIZE: usize = 1;
    pub const TAG_KEY_MAX_SIZE: usize = 128;

    pub const TAG_VALUE_MIN_SIZE: usize = 0;
    pub const TAG_VALUE_MAX_SIZE: usize = 256;

    pub const SESSION_TAGS_MAX_COUNT: usize = 50;
}

pub(crate) mod user {
    pub const USER_NAME_MAX_SIZE: usize = 64;
}
