use std::fs;

pub(crate) fn scrape(url: &str){
    let output = "rust.md";

    println!("Fetching url:{}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("convert html to markdown...");

    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();

    println!("Converted html to markdown saved in {}", output);
}