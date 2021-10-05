use core::fmt;

use crate::game::constants;

#[derive(Debug, Clone)] 
pub struct OauthLoginError;

impl fmt::Display for OauthLoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Copy, Clone)]
pub enum AccountRegion {
    Japan = 1,
    NorthAmerica = 2,
    Europe = 3,
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
    let stored: &str = get_stored(steam_service, region).await?;

    Ok(OauthLoginResult {
        session_id: "0".to_string(),
        can_play: true,
        terms_accepted: true,
        entitled_expansion: 4,
        region: AccountRegion::Japan,
    })
}

async fn get_stored(steam_service: bool, region: AccountRegion) -> Result<&'static str, OauthLoginError>
{
    let url = constants::get_oauth_top_url(&region, false, steam_service);
    println!("{}", url);

    Ok("poop")
}