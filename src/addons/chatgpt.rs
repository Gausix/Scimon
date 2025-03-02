use urlencoding::encode;

use std::error::Error;

use crate::{
    consts::addons::Addons,
    utils::scraping::Scraping,
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
        let scraping = Scraping::new(&self.url);

        let content = scraping.get_html()?;
        let title = scraping.title(&content);
        let html_content = scraping.content(&content, Addons::CHATGPT_CONTENT_CLASS);
    
        Ok((title, html_content))
    }

    pub async fn convert(&self) -> Result<(), Box<dyn Error>> {
        let (file_name, html_content) = self.get_content()?;
        let styled_html = Templates.chatgpt(&html_content);

        let file = format!("{}.pdf", &file_name.replace(" ", "_"));
        let path = format!("{}{}", &self.path, &file);
        let data_url = format!("data:text/html;charset=utf-8,{}", encode(&styled_html));

        Scraping::new(&data_url).print_pdf(path.as_str())?;
        SuccessAlerts::download_and_generated_pdf(&file, &self.url);
        Ok(())
    }

}