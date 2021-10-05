use reqwest::Client;
use crate::util::hex_slice::HexSlice;

lazy_static! {
    static ref CLIENT: Client = reqwest::Client::builder()
        .user_agent(generate_user_agent())
        .build()
        .unwrap();
}

fn generate_user_agent() -> String {
    let mut cid: [u8; 5] = [0, 0xF1, 0xA2, 0x55, 0x02]; // TODO: Generate actual computer ID

    cid[0] = -((cid[1] + cid[2] + cid[3] + cid[4]) as i8) as u8;
    let slice = HexSlice::new(&cid);

    format!("SQEXAuthor/2.0.0(Windows 6.2; ja-jp; {})", slice).to_string()
}