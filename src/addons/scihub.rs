use reqwest;
use std::error::Error;

use scraper::{
    Html, 
    Selector
};

pub struct SciHub {
    url: String,
}

impl SciHub {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub async fn get_pdf(&self) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(&self.url).await?.text().await?;

        let document = Html::parse_document(&response);
        let embed_selector = Selector::parse("div#article embed").unwrap();

        for element in document.select(&embed_selector) {
            if let Some(src) = element.value().attr("src") {
                if src.starts_with("//") {
                    return Ok(
                        format!("https:{}", src)
                    );
                } else {
                    return Ok(
                        format!("https://sci-hub.se{}", src)
                    );
                }
            }
        }

        Err("PDF not found".into())
    } 

}