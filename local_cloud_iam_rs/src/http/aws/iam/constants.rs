pub(crate) mod xml {
    pub(crate) const IAM_XMLNS: &str = "https://iam.amazonaws.com/doc/2010-05-08/";
}

pub(crate) mod tag {
    pub(crate) const MAX_COUNT: usize = 50;
}

pub(crate) mod group {
    pub(crate) const PREFIX: &str = "AIDG";
}

pub(crate) mod policy_version {
    pub(crate) const PREFIX: &str = "ANVA";
    pub(crate) const POLICY_VERSION_MAX_COUNT: usize = 5;
}

pub(crate) mod policy {
    use std::time::Duration;

    pub(crate) const PREFIX: &str = "ANPA";

    /// 15 minutes
    pub(crate) const ROLE_SESSION_DURATION_MIN_SEC: Duration = Duration::new(900, 0);
    /// 12 hours
    pub(crate) const ROLE_SESSION_DURATION_MAX_SEC: Duration = Duration::new(43200, 0);

    pub(crate) const ROLE_SESSION_DEFAULT_DURATION_SEC: Duration = Duration::new(3600, 0);
}

pub(crate) mod user {
    pub(crate) const PREFIX: &str = "AIDA";
}

pub(crate) mod role {
    pub(crate) const PREFIX: &str = "AROA";
    pub(crate) const DEFAULT_MAX_SESSION_DURATION: i32 = 3600;
}

pub(crate) mod instance_profile {
    pub(crate) const PREFIX: &str = "AIPA";
}

pub(crate) mod open_id_connect_provider {
    pub(crate) const URL_PREFIX: &str = "https://";
}

pub(crate) mod mfa {
    pub(crate) const SEED_LENGTH: usize = 20;
    pub(crate) const DEVICE_MAX_COUNT_PER_USER: usize = 8;
}
