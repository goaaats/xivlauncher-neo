use lazy_static::lazy_static;
use reqwest::{Client, IntoUrl, RequestBuilder};

use crate::util;
use crate::util::hex_slice::HexSlice;

use super::{ClientLanguage, platform::{self, Platform}};

lazy_static! {
    static ref LAUNCHER_CLIENT: Client = reqwest::Client::builder()
        .http1_title_case_headers()
        .user_agent(generate_user_agent())
        .build()
        .unwrap();
}

pub fn patch_get(platform: Platform) -> Client {
    let user_agent = platform.get_patch_useragent();

    reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap()
}

pub fn launcher_get<U: IntoUrl>(url: U) -> RequestBuilder {
    LAUNCHER_CLIENT.get(url)
}

pub fn launcher_post<U: IntoUrl>(url: U) -> RequestBuilder {
    LAUNCHER_CLIENT.post(url)
}

pub fn launcher_referer(language: ClientLanguage) -> String {
    format!(
        "https://launcher.finalfantasyxiv.com/v550/index.html?rc_lang={}&time={}",
        language.langcode_underscore(),
        util::time::utc_now_launcher_formatted_long()
    )
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

    format!("SQEXAuthor/2.0.0(Windows 6.2; ja-jp; {})", slice)
}
