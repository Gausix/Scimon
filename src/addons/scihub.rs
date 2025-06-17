use reqwest;
use std::error::Error;

use scraper::{
    Html, 
    Selector
};

use crate::consts::{
    uris::Uris,
    addons::Addons,
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

    pub fn get_doi(&self) -> String {
        let url_str = self.url.replace("https://", "").replace("http://", "").replace(Uris::PROVIDERS_DOMAINS[6], "");
        let last_slices = url_str
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        last_slices[0].to_string() + "/" + last_slices[1]
    } 

    pub fn get_doi_name(&self) -> (String, String) {
        let url_str = self.url.replace("https://", "").replace("http://", "").replace(Uris::PROVIDERS_DOMAINS[6], "");
        let last_slices = url_str
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        (self.url.clone(), last_slices[1].to_string() + ".pdf")
    } 

    pub async fn get_pdf(&self) -> Result<String, Box<dyn Error>> {
        let url = format!("{}{}", Addons::ANNAS_ARCHIVE_ENDPOINT, self.get_doi());
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

        // Err("PDF not found".into())
        Ok("".to_string())
    } 

}