mod oauth {
    enum OauthLoginStatus {
        Success,
    }

    pub struct OauthLoginResult {
        status: OauthLoginStatus,
    }

    pub async fn login(username: &str, password: &str, otp: &str) -> OauthLoginResult {


        OauthLoginResult {
            status: OauthLoginStatus::Success,
        }
    }
}