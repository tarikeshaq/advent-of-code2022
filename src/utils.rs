use std::{env, fs::File, io::prelude::*};

use reqwest::header::HeaderValue;
pub fn read_to_string(day: u32) -> String {
    let file_path = format!("input/day{}.txt", day);

    let mut file = File::open(file_path).unwrap();
    let mut input_txt = String::new();
    file.read_to_string(&mut input_txt).unwrap();
    input_txt
}

pub async fn read_from_server(day: u32) -> String {
    let file_path = format!("input/day{}.txt", day);
    // Either the day hasn't started yet, or this is our first even run!
    // lets be smart and get the input from the website!
    let client = reqwest::Client::new();
    let cookie = env::var("COOKIE").unwrap();
    let header_value = HeaderValue::from_str(&cookie).unwrap();

    let res = client
        .get(&format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("Cookie", header_value)
        .send()
        .await
        .unwrap();
    let input_text = res.text().await.unwrap();
    // lets write it back to the file so we don't do this dance every run
    std::fs::write(&file_path, &input_text).unwrap();
    input_text
}
