use urlencoding::encode;
use headless_chrome::Browser;

use scraper::{
    Html, 
    Selector
};

use std::{
    fs::write,
    error::Error,
};

use crate::{
    consts::addons::Addons,
    generator::templates::Templates,
    ui::success_alerts::SuccessAlerts,
};

pub struct ChatGPT {
    url: String,
    path: String,
}

impl ChatGPT {

    pub fn new(url: &str, path: &str) -> Self {
        Self {
            url: url.to_string(),
            path: path.to_string()
        }
    }

    fn get_content(&self) -> Result<(String, String), Box<dyn Error>> {
        let mut html_content = String::new();

        let browser = Browser::default()?;
        let tab = browser.new_tab()?;

        tab.navigate_to(&self.url)?;
        tab.wait_until_navigated()?;

        let content = tab.get_content()?;
        let document = Html::parse_document(&content);
        let title_selector = Selector::parse("title")?;
        
        let title = document
            .select(&title_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_else(|| String::from("Untitled"));
        
        let selector = Selector::parse(Addons::CHATGPT_CONTENT_CLASS)?;
        for element in document.select(&selector) {
            html_content.push_str(&element.inner_html());
        }
    
        Ok((title, html_content))
    }

    pub async fn convert(&self) -> Result<(), Box<dyn Error>> {
        let (file_name, html_content) = self.get_content()?;
        let styled_html = Templates.chat_gpt(&html_content);

        let file = format!("{}.pdf", &file_name);
        let path = format!("{}{}", &self.path, &file);
        let data_url = format!("data:text/html;charset=utf-8,{}", encode(&styled_html));
    
        let browser = Browser::default()?;
        let tab = browser.new_tab()?;
    
        tab.navigate_to(&data_url)?;
        tab.wait_until_navigated()?;
    
        let contents = tab.print_to_pdf(None)?;
        write(&path, contents)?;
    
        SuccessAlerts::download_and_generated_pdf(&file, &self.url);
        Ok(())
    }    

}