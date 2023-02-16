use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

const CRLF: &str = "\r\n";
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_connection(stream));
    }
}

/**
 * rustc main.rs -C opt-level=3 -C debuginfo=0
 * ./main
 * http://127.0.0.1:8080
 * npm install -g loadtest
 * loadtest -t 10 -c 100 "http://127.0.0.1:8080/"
 * 简单解释下命令：
-t 10 代表测试最长进行10秒
-c 100 代表并发数100
"http://127.0.0.1:8080/" 测试网址（这里必须以/结尾）
 */
// 首页
fn handle_index() -> (String, String) {
    (file_return("index.html"), status(200, "OK"))
}

// 404页面
fn handle_404() -> (String, String) {
    (String::new(), status(200, "OK"))
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer= [0; 4096];
    stream.read(&mut buffer).unwrap();

    let _matched = |route: &str| matched(&buffer, route);
    let _write = |(contents, status)| write(stream, contents, status);

    // 路由处理
    if _matched("/") {
        _write(handle_index());
    } else {
        _write(handle_404());
    }
}

// 路由匹配
fn matched(buffer: &[u8; 4096], route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    buffer.starts_with(s.as_bytes())
}

// 读取本地文件内容
fn file_return(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn status(code: i32, text: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, text, CRLF)
}

// 将响应写出到流
fn write(mut stream: TcpStream, contents: String, status: String) {
    let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
    let server = format!("Server: Rust{}", CRLF);
    let content_length = format!("Content-Length: {}{}", contents.as_bytes().len(), CRLF);
    let response = format!(
        "{0}{1}{2}{3}{4}{5}",
        status, server, content_type, content_length, CRLF, contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
