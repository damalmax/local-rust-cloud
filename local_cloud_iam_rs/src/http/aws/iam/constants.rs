pub(crate) mod xml {
    pub(crate) const IAM_XMLNS: &str = "https://iam.amazonaws.com/doc/2010-05-08/";
}

pub(crate) mod marker {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub(crate) static ref MARKER_REGEX: Regex = Regex::new(r"^[\\u0020-\\u00FF]+$").unwrap();
    }

    pub(crate) const MARKER_MIN_SIZE: usize = 1;
    pub(crate) const MARKER_MAX_SIZE: usize = 320;
}

pub(crate) mod arn {
    pub(crate) const ARN_MIN_LENGTH: usize = 20;
    pub(crate) const ARN_MAX_LENGTH: usize = 2048;
}

pub(crate) mod account {
    pub(crate) const ACCOUNT_ALIAS_MIN_SIZE: usize = 3;
    pub(crate) const ACCOUNT_ALIAS_MAX_SIZE: usize = 63;
}

pub(crate) mod policy_version {
    pub(crate) const POLICY_VERSION_PREFIX: &str = "ANVA";
}

pub(crate) mod policy {
    use std::time::Duration;

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub(crate) static ref POLICY_PATH_REGEX: Regex = Regex::new(r"^((/[A-Za-z0-9\.,\+@=_-]+)*)/$").unwrap();
        pub(crate) static ref POLICY_PATH_PREFIX_REGEX: Regex = Regex::new(r"^\\u002F[\\u0021-\\u007F]*$").unwrap();
        pub(crate) static ref POLICY_NAME_VALID_CHARACTERS: Vec<char> =
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+=,.@-_"
                .chars()
                .into_iter()
                .collect();
        pub(crate) static ref POLICY_DESCRIPTION_VALID_CHARACTERS: Vec<char> =
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+=,.@-_ "
                .chars()
                .into_iter()
                .collect();
    }

    pub(crate) const MANAGED_POLICY_PREFIX: &str = "ANPA";

    pub(crate) const INLINE_USER_POLICY_MAX_SIZE: usize = 2048;
    pub(crate) const INLINE_ROLE_POLICY_MAX_SIZE: usize = 10240;
    pub(crate) const INLINE_GROUP_POLICY_MAX_SIZE: usize = 5120;

    pub(crate) const MANAGED_POLICY_MAX_SIZE: usize = 6144;
    pub(crate) const MANAGED_POLICIES_PER_SESSION_MAX_COUNT: usize = 10;

    pub(crate) const POLICY_NAME_MAX_LENGTH: usize = 128;
    pub(crate) const POLICY_NAME_MIN_LENGTH: usize = 1;
    pub(crate) const POLICY_DESCRIPTION_MAX_LENGTH: usize = 1000;
    pub(crate) const PATH_MIN_LENGTH: usize = 1;
    pub(crate) const PATH_MAX_LENGTH: usize = 512;
    pub(crate) const ROLE_NAME_MAX_LENGTH: usize = 64;

    /// 15 minutes
    pub(crate) const ROLE_SESSION_DURATION_MIN_SEC: Duration = Duration::new(900, 0);
    /// 12 hours
    pub(crate) const ROLE_SESSION_DURATION_MAX_SEC: Duration = Duration::new(43200, 0);

    pub(crate) const ROLE_SESSION_DEFAULT_DURATION_SEC: Duration = Duration::new(3600, 0);
}

pub(crate) mod tag {
    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        pub(crate) static ref TAG_KEY_REGEX: Regex = Regex::new(r"^[\p{L}\p{Z}\p{N}_.:/=+\-@]+$").unwrap();
        pub(crate) static ref TAG_VALUE_REGEX: Regex = Regex::new(r"^[\p{L}\p{Z}\p{N}_.:/=+\-@]*$").unwrap();
    }

    pub(crate) const TAG_KEY_MIN_LENGTH: usize = 1;
    pub(crate) const TAG_KEY_MAX_LENGTH: usize = 128;

    pub(crate) const TAG_VALUE_MAX_LENGTH: usize = 256;

    pub(crate) const SESSION_TAGS_MAX_COUNT: usize = 50;
}

pub(crate) mod user {
    pub(crate) const USER_NAME_MAX_SIZE: usize = 64;
}
