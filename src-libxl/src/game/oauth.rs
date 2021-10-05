use core::fmt;

#[derive(Debug, Clone)] 
pub struct OauthLoginError;

impl fmt::Display for OauthLoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

pub enum AccountRegion {
    Japan,
    NorthAmerica,
    Europe,
}

pub struct OauthLoginResult {
    session_id: String,
    can_play: bool,
    terms_accepted: bool,
    entitled_expansion: i16,
    region: AccountRegion
}

/// Login to Square Enix oauth with the supplied credentials
pub async fn login(username: &str, password: &str, otp: &str, steam_service: bool, region: AccountRegion) -> Result<OauthLoginResult, OauthLoginError> {
    Ok(OauthLoginResult {
        session_id: "".to_string(),
        can_play: true,
        terms_accepted: true,
        entitled_expansion: 4,
        region: AccountRegion::Japan,
    })
}
