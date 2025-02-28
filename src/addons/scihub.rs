use reqwest;
use std::error::Error;

use scraper::{
    Html, 
    Selector
};

use crate::consts::uris::Uris;

pub struct SciHub {
    url: String,
}

impl SciHub {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    fn complete_url(&self, src: &str) -> String {
        if src.starts_with("//") {
            format!("https:{}", src)
        } else {
            format!("https://{}{}", Uris::PROVIDERS_DOMAINS[6], src)
        }
    }

    pub async fn get_pdf(&self) -> Result<String, Box<dyn Error>> {
        let response = reqwest::get(&self.url).await?.text().await?;

        let document = Html::parse_document(&response);
        let embed_selector = Selector::parse("div#article embed").unwrap();

        for element in document.select(&embed_selector) {
            if let Some(src) = element.value().attr("src") {
                return Ok(self.complete_url(src))
            }
        }

        Err("PDF not found".into())
    } 

}