enum OauthLoginResult {
    Success,
}

impl Oauth {
    pub async fn login(username: &str, password: &str, otp: &str) -> OauthLoginResult {
        OauthLoginResult::Success
    }
}