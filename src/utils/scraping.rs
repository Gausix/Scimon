use std::error::Error;
use headless_chrome::Browser;

use scraper::{
    Html, 
    Selector
};

pub struct Scraping {
    url: String,
}

impl Scraping {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn get_html(&self) -> Result<String, Box<dyn Error>> {
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to(&self.url)?;
        tab.wait_until_navigated()?;

        let content = tab.get_content()?;
        Ok(content)
    }

    pub fn title(&self, content: &str) -> String {
        let document = Html::parse_document(&content);

        let title_selector = match Selector::parse("title") {
            Ok(selector) => selector,
            Err(_) => return String::from("Untitled"),
        };
        
        return document
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
                .unwrap_or_else(|| String::from("Untitled"));
    }

    pub fn content(&self, content: &str, class: &str) -> String {
        let document = Html::parse_document(&content);

        let selector = match Selector::parse(class) {
            Ok(selector) => selector,
            Err(_) => return String::from(""),
        };

        let mut html_content = String::new();

        for element in document.select(&selector) {
            html_content.push_str(&element.inner_html());
        }

        return html_content;
    }

}