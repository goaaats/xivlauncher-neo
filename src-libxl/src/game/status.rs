use serde::Deserialize;

use crate::game::constants::{get_frontier_gate_status_url, get_frontier_login_status_url};

#[derive(Deserialize)]
struct FrontierStatus {
    status: i8,
}

async fn get_status(url: String) -> Result<bool, reqwest::Error> {
    let resp = reqwest::get(url).await?;

    let res: FrontierStatus = resp.json::<FrontierStatus>().await?;
    Ok(res.status == 1)
}

pub async fn is_gate() -> Result<bool, reqwest::Error> {
    get_status(get_frontier_gate_status_url()).await
}

pub async fn is_login() -> Result<bool, reqwest::Error> {
    get_status(get_frontier_login_status_url()).await
}
