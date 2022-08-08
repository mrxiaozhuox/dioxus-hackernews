use reqwasm::http::Request;

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