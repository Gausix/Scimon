use reqwest;
use std::error::Error;

use scraper::{
    Html, 
    Selector
};

use crate::{
    system::doi::DOI,
    consts::addons::Addons,
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
        let doi = DOI::new(&self.url);
        let url = format!("{}{}", Addons::ANNAS_ARCHIVE_ENDPOINT, doi.scihub());
        let response = reqwest::get(&url).await?.text().await?;

        let document = Html::parse_document(&response);
        let selector = Selector::parse("li a").unwrap();

        for element in document.select(&selector) {
            if let Some(text) = element.text().next() {
                if text.contains("Download") {
                    if let Some(href) = element.value().attr("href") {
                        return Ok(href.to_string());
                    }
                }
            }
        }

       Err("PDF not found".into())
    }

}