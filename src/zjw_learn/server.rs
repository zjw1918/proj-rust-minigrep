extern crate minigrep;

use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use minigrep::ThreadPool;

pub fn run() {
  let address = "127.0.0.1:7878";
  let listener = TcpListener::bind(address).unwrap();
  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    pool.execute(|| {
      // println!("Connection established");
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer = [0; 512];
  stream.read(&mut buffer).unwrap();
  // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

  let get = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";

  let (status_line, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
  } else if buffer.starts_with(sleep) {
    thread::sleep(Duration::from_secs(5));
    ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
  };

  let mut f = File::open(filename).unwrap();
  let mut contents = String::new();
  f.read_to_string(&mut contents).unwrap();

  // 编写一个响应
  // let response = "HTTP/1.1 200 OK\r\n\r\n";
  let response = format!("{}{}", status_line, contents);
  stream.write(response.as_bytes()).unwrap();
  stream.flush().unwrap();
}