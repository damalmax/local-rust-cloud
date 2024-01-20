pub(crate) mod xml {
    pub(crate) const IAM_XMLNS: &str = "https://iam.amazonaws.com/doc/2010-05-08/";
}

pub(crate) mod policy_version {
    pub(crate) const POLICY_VERSION_PREFIX: &str = "ANVA";
}

pub(crate) mod policy {
    use std::time::Duration;

    pub(crate) const MANAGED_POLICY_PREFIX: &str = "ANPA";

    /// 15 minutes
    pub(crate) const ROLE_SESSION_DURATION_MIN_SEC: Duration = Duration::new(900, 0);
    /// 12 hours
    pub(crate) const ROLE_SESSION_DURATION_MAX_SEC: Duration = Duration::new(43200, 0);

    pub(crate) const ROLE_SESSION_DEFAULT_DURATION_SEC: Duration = Duration::new(3600, 0);
}
