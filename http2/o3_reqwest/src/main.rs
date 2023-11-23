use reqwest::{Client, Version};
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::builder().http2_prior_knowledge().build()?;
    let response = client.get("https://www.douyin.com").send().await?;

    match response.version() {
        Version::HTTP_11 => println!("Used HTTP/1.1"),
        Version::HTTP_2 => println!("Used HTTP/2"),
        _ => println!("Used another version"),
    }

    Ok(())
}
