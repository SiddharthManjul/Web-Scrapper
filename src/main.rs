use reqwest;
use scraper::{selector, Html, Selector};
use printpdf::*;
use tokio::join;
use std::fs::File;
use std::io::BufWriter;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://doc.rust-lang.org/book/";
    let response = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("body").unwrap();

    let mut text_content = String::new();

    for element in document.select(&selector) {
        text_content.push_str(&element.text().collect::<Vec<_>>().join(" "));
    }

    Ok(())
}
