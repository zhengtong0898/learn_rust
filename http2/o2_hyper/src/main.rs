use hyper::body::to_bytes;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;
use std::error::Error;
use tokio_native_tls::TlsConnector;

const ALPN_H2: &str = "h2";

fn build_tls_connector(is_http2: bool) -> Result<TlsConnector, Box<dyn Error>> {
    let mut builder = native_tls::TlsConnector::builder();

    if is_http2 {
        builder.request_alpns(&[ALPN_H2]);
    }

    let connector = builder.build()?;
    Ok(TlsConnector::from(connector))
}

#[tokio::main]
async fn main() {
    // 创建 HttpsConnector
    let mut http = HttpConnector::new();
    http.enforce_http(false);
    let tls_connector =
        build_tls_connector(true).unwrap_or_else(|e| panic!("init tls_connector failed: {}", e));
    let https = HttpsConnector::from((http, tls_connector));

    // 创建 HTTP/2 客户端
    let client = Client::builder()
        .http2_only(true) // 强制使用 HTTP/2
        .build::<_, hyper::Body>(https);

    // 目标 URL
    let uri: Uri = "https://zhuanlan.zhihu.com/p/628470666".parse().unwrap();

    let request = Request::builder().uri(uri).body(Body::empty()).unwrap();
    let response = client.request(request).await;

    // 发起请求
    match response {
        Ok(res) => {
            println!("Response: {}", res.status());
            let bytes = to_bytes(res.into_body())
                .await
                .unwrap_or_else(|e| panic!("body into bytes failed: {}", e));
            let content = String::from_utf8(bytes.to_vec());
            println!("Response: {}", content.unwrap());
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
