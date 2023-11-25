use reqwest::header;
use reqwest::{Client, Version};
use tokio;

// 通过配置Cargo.toml, 开启 ["rustls-tls", "gzip", "deflate", "brotli", "cookies"]
// "rustls-tls" 和 default-features = false: 使支持http2
// "gzip", "deflate", "brotli":              使支持压缩数据和解压数据
// "cookies":                                使支持cookie存储

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let form_data = [("u", "u1111"), ("p", "p1111")];
    let accept_encoding = "gzip, deflate, br";
    let user_agent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36";

    let client = Client::builder()
        .use_rustls_tls()
        .cookie_store(true) // 开启cookie存储功能
        .build()
        .unwrap();

    let response = client
        .post("https://passport.jd.com/uc/loginService")
        .header(header::USER_AGENT, user_agent)
        .header(header::ACCEPT_ENCODING, accept_encoding) // 支持压缩和解压缩数据
        .form(&form_data)
        .send()
        .await?;

    match response.version() {
        Version::HTTP_11 => println!("Used HTTP/1.1"),
        Version::HTTP_2 => println!("Used HTTP/2"),
        _ => println!("Used another version"),
    }

    Ok(())
}
