use reqwest::header;
use reqwest::{Client, Version};
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36";

    let proxy = reqwest::Proxy::all("http://127.0.0.1:7890").unwrap(); // 配置代理
    let client = Client::builder()
        .cookie_store(true)
        .proxy(proxy) // 开启代理
        .build()
        .unwrap();

    let response = client
        .get("https://www.google.com")
        .header(header::USER_AGENT, user_agent)
        .send()
        .await?;

    match response.version() {
        Version::HTTP_11 => println!("Used HTTP/1.1"),
        Version::HTTP_2 => println!("Used HTTP/2"),
        _ => println!("Used another version"),
    }

    println!("response.text(): {:?}", response.text().await?);

    Ok(())
}
