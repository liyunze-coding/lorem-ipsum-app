use dotenv;
use reqwest::Error;
use serde::Deserialize;

// allow never read
#[derive(Debug, Deserialize)]
pub struct IpsumResponse {
    #[allow(dead_code)]
    pub text: String,
}

impl IpsumResponse {
    pub fn get_text(&self) -> &str {
        &self.text
    }
}

pub async fn get_ipsum(paragraphs: u32) -> Result<IpsumResponse, Error> {
    let url = format!("https://api.api-ninjas.com/v1/loremipsum?paragraphs={paragraphs}");

    let api_key = dotenv::var("API_KEY").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("X-Api-Key", api_key)
        .send()
        .await?
        .json::<IpsumResponse>()
        .await?;

    Ok(res)
}
