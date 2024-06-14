### isahc 如何打印网络请求

1. 在 isahc 的 src/handlers.rs:L658 -> impl curl::easy::Handler for RequestHandler -> fn debug
2. 运行时需要开启 debug 模式.  
```
 impl curl::easy::Handler for RequestHandler {

    fn debug(&mut self, kind: InfoType, data: &[u8]) {
        let _enter = self.span.enter();

        struct FormatAscii<T>(T);

        impl<T: AsRef<[u8]>> fmt::Display for FormatAscii<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for &byte in self.0.as_ref() {
                    ascii::escape_default(byte).fmt(f)?;
                }
                Ok(())
            }
        }

        match kind {
            InfoType::Text => {
                tracing::debug!("{}", String::from_utf8_lossy(data).trim_end())
            }
            InfoType::HeaderIn | InfoType::DataIn => {
                tracing::trace!(target: "isahc::wire", "<< {}", FormatAscii(data))
            }
            InfoType::HeaderOut | InfoType::DataOut => {
                println!("{}", String::from_utf8_lossy(data).trim_end());                         // 增加这一行代码
                tracing::trace!(target: "isahc::wire", ">> {}", FormatAscii(data))
            }
            _ => (),
        }
    }

}
```

输出结果
```
GET /v2/topnews?page=1&type=2 HTTP/2
Host: www.baidu.com
Accept-Encoding: deflate, gzip
user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36
accept: application/json
```