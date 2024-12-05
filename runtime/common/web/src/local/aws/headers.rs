macro_rules! headers {
    (
        $(#[$m:meta])*
        enum $name:ident {
            $($variant:ident($str:tt)),+
            $(,)?
        }
    ) => {
        $(#[$m])*
        #[non_exhaustive]
        pub enum $name {
            $(
                $variant
            ),+
        }

        impl $name {
            pub fn as_str(&self) -> &str {
                match self {
                    $(
                        $name::$variant => $str,
                    )+
                }
            }
        }
    };
}

headers! {
    #[derive(Debug)]
    enum AwsRequestHeaders {
        Action("Action"),
        Version("Version"),
        AmzAlgorithm("X-Amz-Algorithm"),
        AmzCredential("X-Amz-Credential"),
        AmzContentSha256("X-Amz-Content-SHA256"),
        AmzDate("X-Amz-Date"),
        AmzExpires("X-Amz-Expires"),
        AmzSecurityToken("X-Amz-Security-Token"),
        AmzSignature("X-Amz-Signature"),
        AmzSignedHeaders("X-Amz-SignedHeaders"),
        AmzUserAgent("X-Amz-User-Agent"),
    }
}
