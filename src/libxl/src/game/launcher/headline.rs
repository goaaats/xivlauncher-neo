use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::{game::{ClientLanguage, request}, util::time};

#[derive(Debug)]
pub enum HeadlineError {
    Reqwest(reqwest::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeadlineEntry {
    pub date: DateTime<Utc>,
    pub title: String,
    pub url: String,
    pub id: String,
    pub tag: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BannerEntry {
    pub lsb_banner: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Headline {
    pub news: Vec<HeadlineEntry>,
    pub topics: Vec<HeadlineEntry>,
    pub pinned: Vec<HeadlineEntry>,
    pub banner: Vec<BannerEntry>,
}

impl Headline {
    pub async fn get(language: ClientLanguage) -> Result<Self, HeadlineError> {
        let url = format!("https://frontier.ffxiv.com/news/headline.json?lang={}&media=pcapp&{}", language.langcode(), time::utc_now_millis());
        let resp = request::launcher_get(&url).send().await.map_err(HeadlineError::Reqwest)?;
        Ok(resp.json().await.map_err(HeadlineError::Reqwest)?)
    }
}