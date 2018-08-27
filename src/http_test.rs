extern crate reqwest;

use std::error::Error;
use std::io::Read;
use self::reqwest::header::Headers;

const ROOT_URL: &str = "https://f82ocash.api.lncld.net/1.1/";

#[derive(Deserialize)]
#[derive(Debug)]
struct LeanResult {
    objectId: String,
    ringData: String,
}

pub fn run() {
    test_request_json();
}

fn test_request_json() {
    let mut headers = Headers::new();
    headers.set_raw("X-LC-Id", "f82OcAshk5Q1J993fGLJ4bbs-gzGzoHsz");
    headers.set_raw("X-LC-Key", "O9COJzi78yYXCWVWMkLqlpp8");
    let client = reqwest::Client::new();
    let url = format!("{}classes/RingSport/5b83a304ee920a003b77b754", ROOT_URL);
    let res: LeanResult = client.get(url.as_str())
        .headers(headers)
        .send().unwrap().json().unwrap();
    // let mut body = String::new();
    // res.read_to_string(&mut body).unwrap();
    println!("{:?}", res);
}
