use std::vec;

use reqwasm::http::Request;
use serde::{Serialize, Deserialize};

const BASE_URL: &'static str = "https://hacker-news.firebaseio.com/v0/";

pub async fn get_list(name: &str) -> Vec<u64> {
    let url = format!("{BASE_URL}/{name}.json");
    let resp = Request::get(&url).send().await;
    if resp.is_err() {
        return vec![];
    }
    let resp = resp.unwrap();
    resp.json::<Vec<u64>>().await.unwrap_or_default()
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Item {
    pub id: u64,
    pub by: String,
    pub descendants: u32,
    pub kids: Vec<u64>,
    pub score: u32,
    pub time: i64,
    pub title: String,
    pub r#type: String,
    pub url: String,
}

pub async fn get_item(id:u64) -> Option<Item> {
    let url = format!("{BASE_URL}/item/{id}.json");
    let resp = Request::get(&url).send().await.ok()?;
    let data = resp.json::<Item>().await.ok()?;
    Some(data)
}

pub const ONE_PAGE_ITEM_NUM: usize = 20;
pub fn split_page(full: Vec<u64>, current_page: usize) -> Vec<u64> {
    if full.len() < (current_page * ONE_PAGE_ITEM_NUM) {
        return vec![];
    }
    if current_page == 1 {
        return full[0..ONE_PAGE_ITEM_NUM].to_vec();
    }
    let start = ONE_PAGE_ITEM_NUM * current_page;
    full[(start - ONE_PAGE_ITEM_NUM)..start].to_vec()
}

pub fn max_page_num(full: Vec<u64>) -> usize {
    let t = full.len() as f32 % ONE_PAGE_ITEM_NUM as f32;
    if t > 0.0 {
        full.len() / ONE_PAGE_ITEM_NUM + 1
    } else {
        full.len() / ONE_PAGE_ITEM_NUM
    }
}