use scraper::{
    Html, 
    Selector
};

pub struct Scraping;

impl Scraping {

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

}