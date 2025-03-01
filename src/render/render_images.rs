use reqwest;

use scraper::{
    Html, 
    Selector
};

use base64::{
    Engine as _,
    engine::general_purpose, 
};

use std::error::Error;

pub struct RenderImages {
    content: String,
}

impl RenderImages {

    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }

    pub async fn render(&mut self) -> Result<String, Box<dyn Error>> {
        let html = Html::parse_document(&self.content);
        let selector = Selector::parse("img").unwrap();

        for img in html.select(&selector) {
            let src = img.value().attr("src").unwrap();
            let src = src.to_string();

            if src.starts_with("http") {
                let response = reqwest::get(&src).await?;

                let content_type = response
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|ct| ct.to_str().ok())
                    .unwrap_or("image/jpeg")
                    .to_string();

                let img = reqwest::get(&src).await?.bytes().await?;
                let img = general_purpose::STANDARD.encode(&img);
                let img = format!("data:{};base64,{}", content_type, img);

                self.content = self.content.replace(&src, &img);
            }
        }

        Ok(self.content.clone())
    }

}