use dotenv;

pub async fn get_ipsum(paragraphs: u32) -> Result<String, reqwest::Error> {
    let url = format!("https://api.api-ninjas.com/v1/loremipsum?paragraphs={paragraphs}");

    let api_key = dotenv::var("API_KEY").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .get(url)
        .header("X-Api-Key", api_key)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}
