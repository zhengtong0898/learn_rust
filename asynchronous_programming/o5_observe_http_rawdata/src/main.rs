use reqwest;
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    let url = format!("https://www.example.com/login");
    let form_data = [("username", "zhangsan"), ("password", "123456")];

    // http 1.1
    // tokio-1.34.0: src/io/async_write.rs:deref_async_write:193:
    // "POST /login HTTP/1.1\r\ncontent-type: application/x-www-form-urlencoded\r\naccept: */*\r\naccept-encoding: gzip, br, deflate\r\nhost: www.example.com\r\ncontent-length: 33\r\n\r\nusername=zhangsan&password=123456
    let client = Client::builder()
        .cookie_store(true)
        .http1_only()
        .build()
        .unwrap();
    let request = client.post(url.clone()).form(&form_data).build().unwrap();
    let response = client.execute(request).await.unwrap();

    println!("Hello, world!: {:?}", response.version());
    println!("Hello, world!: {:?}", response.status());
    println!("Hello, world!: {:?}", response.text().await);

    // http 2.0
    //
    // 建立tls连接: tcpstream handshake...
    //
    // 建立http2连接
    // tokio-1.34.0: src/io/async_write.rs:deref_async_write:193: "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"
    // tokio-1.34.0: src/io/async_write.rs:deref_async_write:193: "PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n"
    // tokio-1.34.0: src/io/async_write.rs:deref_async_write:193: "\0\0\u{12}\u{4}\0\0\0\0\0\0\u{2}\0\0\0\0\0\u{4}\0 \0\0\0\u{5}\0\0@\0\0\0\u{4}\u{8}\0\0\0\0\0\0O\0\u{1}"
    //
    // 发送header
    // h2-0.3.22: src/proto/stream/send.rs:140: send_headers:
    // {"content-type": "application/x-www-form-urlencoded", "accept": "*/*", "accept-encoding": "gzip, br, deflate", "content-length": "33"}
    //
    // 发送body
    // tokio-1.34.0: src/io/async_write.rs:deref_async_write:193:
    // "\0\0z\u{1}\u{4}\0\0\0\u{1}��A��������yr\u{1e}�\u{4}�b��\u{19}lY\u{5}���V;��\u{f}1��(9\u{7}A`�<W��E\0]\0B�\t��k���=�\u{b}!jI�B M��_�\u{1d}u�b\r&=Ly[Ǐ\u{b}J{)Z�(-D<��S��c�P��٫�R;>����4�\u{f}\r�m�\0\0;\0\u{1}\0\0\0\u{1}username=zhangsan&password=123456"
    let client = Client::builder().cookie_store(true).build().unwrap();
    let request = client.post(url).form(&form_data).build().unwrap();
    let response = client.execute(request).await.unwrap();

    println!("Hello, world!: {:?}", response.version());
    println!("Hello, world!: {:?}", response.status());
    println!("Hello, world!: {:?}", response.text().await);
}
