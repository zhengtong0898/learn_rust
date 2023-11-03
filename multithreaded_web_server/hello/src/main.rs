use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // stream 是一个通信对象, 可以做 connect、read、write 操作.
    // 这里之所以要使用BufReader来接手stream, 是因为BufReader提供了lines这个按行read数据接口方法,
    // 如果不使用BufReader, 那么你需要一个字节一个字节的去读取stream并处理行的边界.
    let mut buf_reader = BufReader::new(&mut stream);

    // 这段代码的详细解释在下面的handle_connection2函数体内.
    let mut content_length = 0;
    let http_request: Vec<_> = buf_reader
        .by_ref() // Borrow the buf_reader instead of moving it.
        .lines()
        .map(|result| {
            let line = result.unwrap();
            if line.starts_with("Content-Length:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                content_length = parts[1].parse::<usize>().unwrap_or(0);
            }
            line
        })
        .take_while(|line: &String| !line.is_empty())
        .collect();

    // 打印 Request
    println!("Request: {:#?}", http_request);
    if content_length > 0 {
        let mut body = String::new();
        buf_reader
            .take(content_length as u64)
            .read_to_string(&mut body)
            .unwrap();
        println!("Body: {}", body);
    }

    // 返回一个html内容
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

#[allow(dead_code)]
fn handle_connection2(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    {
        // 1. 从buf_reader中获取行迭代器
        let lines_iter = buf_reader.lines();

        // 初始化一个空的Vector来存储请求
        let mut http_request = Vec::new();

        // 2. 遍历每一行
        for line_result in lines_iter {
            // 使用map的功能进行unwrap
            let line = match line_result {
                Ok(l) => l,
                Err(e) => {
                    // 在这里，我们打印错误并停止迭代。
                    // 您也可以选择处理错误的其他方式。
                    println!("Error reading line: {}", e);
                    break;
                }
            };

            // 3. 使用take_while的功能进行检查
            if line.is_empty() {
                break;
            }

            // 4. 将读取的行添加到Vector中
            http_request.push(line);
        }

        println!("Request: {:#?}", http_request);
    }
}
