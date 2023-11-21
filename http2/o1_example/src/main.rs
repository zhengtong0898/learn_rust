use bytes;
use h2::client;
use http::{Method, Request, Uri};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio_rustls::rustls::{ClientConfig, OwnedTrustAnchor, RootCertStore, ServerName};
use tokio_rustls::TlsConnector;

use std::convert::TryFrom;
use std::error::Error;

const ALPN_H2: &str = "h2";

fn build_tls_connector(is_http2: bool) -> Result<TlsConnector, Box<dyn Error>> {
    let mut root_store = RootCertStore::empty();

    root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
        OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject,
            ta.spki,
            ta.name_constraints,
        )
    }));

    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    if is_http2 {
        config.alpn_protocols.push(ALPN_H2.as_bytes().to_owned());
    }

    Ok(TlsConnector::from(Arc::new(config)))
}

async fn build_tls_client(
    uri_str: &str,
    is_http2: bool,
) -> Result<client::SendRequest<bytes::Bytes>, Box<dyn Error>> {
    let (domain, port) = parse_addr_from_uri(uri_str)
        .unwrap_or_else(|e| panic!("parse addr from uri failed: {}", e));

    let tcp = TcpStream::connect(format!("{domain}:{port}"))
        .await
        .unwrap_or_else(|e| panic!("init tcp_stream failed: {}", e));

    let dns_name = ServerName::try_from(domain.as_str())
        .unwrap_or_else(|e| panic!("init dns_name failed: {}", e));

    let tls_connector = build_tls_connector(is_http2)
        .unwrap_or_else(|e| panic!("init tls_connector failed: {}", e));

    let tls_stream = TlsConnector::from(tls_connector)
        .connect(dns_name, tcp)
        .await
        .unwrap_or_else(|e| panic!("init tls_stream failed: {}", e));

    {
        let (_, session) = tls_stream.get_ref();
        let negotiated_protocol = session.alpn_protocol();
        assert_eq!(Some(ALPN_H2.as_bytes()), negotiated_protocol);
    }

    println!("Starting client handshake");
    let (client, h2) = client::handshake(tls_stream).await?;

    tokio::spawn(async move {
        if let Err(e) = h2.await {
            println!("GOT ERR={:?}", e);
        }
    });

    Ok(client)
}

fn parse_addr_from_uri(uri_str: &str) -> Result<(String, i32), Box<dyn Error>> {
    let uri: Uri = uri_str.parse().expect("parse uri_str to Uri failed");
    if let Some(authority) = uri.authority() {
        let domain = authority.host();
        let port = authority.port_u16().unwrap_or_else(|| {
            if uri.scheme_str() == Some("https") {
                443 // 默认 HTTPS 端口
            } else {
                80 // 默认 HTTP 端口
            }
        });

        Ok((domain.to_string(), port as i32))
    } else {
        Err("URI does not have an authority component".into())
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let uri = "https://www.douyin.com";

    // "building request"
    let request = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(())
        .unwrap();

    // sending request
    let mut client = build_tls_client(uri, true).await.unwrap();
    let (response, _) = client.send_request(request, true).unwrap();

    // waiting on response
    let (resp, mut body) = response.await.unwrap().into_parts();

    // processing body
    println!("resp: {}", resp.status);
    println!("resp: {:?}", resp.headers);
    while let Some(chunk) = body.data().await {
        println!("RX: {:?}", chunk?);
    }

    Ok(())
}
