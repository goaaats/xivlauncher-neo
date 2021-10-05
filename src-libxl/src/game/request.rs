use reqwest::{Client, IntoUrl, RequestBuilder};

use crate::game::constants;
use crate::util;
use crate::util::hex_slice::HexSlice;

use super::ClientLanguage;

lazy_static! {
    static ref LAUNCHER_CLIENT: Client = reqwest::Client::builder()
        .user_agent(generate_user_agent())
        .build()
        .unwrap();
    static ref PATCH_CLIENT: Client = reqwest::Client::builder()
        .user_agent(constants::PATCH_CLIENT_USER_AGENT)
        .build()
        .unwrap();
}

pub fn launcher_get<U: IntoUrl>(url: U) -> RequestBuilder {
    LAUNCHER_CLIENT.get(url)
}

pub fn get_launcher_referer(language: ClientLanguage) -> String {
    format!(
        "https://launcher.finalfantasyxiv.com/v550/index.html?rc_lang={}&time={}",
        language.get_langcode_underscore(),
        util::get_launcher_formatted_time_long()
    )
    .to_string()
}

fn generate_user_agent() -> String {
    let mut cid: [u8; 5] = [0, 0xF1, 0xA2, 0x55, 0x02]; // TODO: Generate actual computer ID

    let mut temp = cid[1].wrapping_add(cid[2]);
    temp = temp.wrapping_add(cid[3]);
    temp = temp.wrapping_add(cid[4]);

    temp = temp.wrapping_neg();

    //cid[0] = -(cid[1] + cid[2] + cid[3] + cid[4]);
    cid[0] = temp;
    let slice = HexSlice::new(&cid);

    format!("SQEXAuthor/2.0.0(Windows 6.2; ja-jp; {})", slice).to_string()
}
