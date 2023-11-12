use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_sts::config::Region;

use super::*;

#[actix_rt::test]
async fn assume_role() {
    let mut ctx = TestContext::new().await;
    let port = ctx.port;
    let config = aws_config::SdkConfig::builder()
        .region(Some(Region::new("eu-local-1")))
        .endpoint_url(format!("http://localhost:{}/sts", port))
        .credentials_provider(SharedCredentialsProvider::new(credentials_provider()))
        .build();
    let client = aws_sdk_sts::Client::new(&config);

    let test_role_session_name = "s3_access_example";

    let response = client
        .assume_role()
        .role_arn("arn:aws:sts::000000000000:role/rd_role")
        .role_session_name(test_role_session_name)
        .send()
        .await
        .expect("Failed to assume role");

    let assumed_role_id = response
        .assumed_role_user()
        .expect("AssumedRoleUser property should be available in the response")
        .assumed_role_id()
        .expect("AssumedRoleId property should be available in the response");

    let parts = assumed_role_id.split(":");

    assert_eq!(test_role_session_name, parts.enumerate().last().unwrap().1);
    ctx.stop_server().await;
}
