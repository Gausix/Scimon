extern crate reqwest;

use std::error::Error;

use serde::Deserialize;

use crate::{
    consts::uris::Uris,
    cmd::download::Download,
    ui::errors_alerts::ErrorsAlerts,
};

#[derive(Debug, Deserialize)]
struct Item {
    url: String,
    encrypted: bool,
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    total: Option<u32>,

    #[serde(default)]
    list: Vec<Item>,

    #[serde(default)]
    message: String,

    success: Option<bool>
}

pub struct Scrape;

impl Scrape {

    async fn fetch_items(url: &str) -> Result<Response, Box<dyn std::error::Error>> {
        let url_with_param = Uris::PAIMON_SCRAPE_API_REQUEST.to_string() + url;
        let response = reqwest::get(&url_with_param).await?;
        let body = response.bytes().await?;
        
        let data = serde_json::from_slice(&body)?;
        Ok(data)
    }

    pub async fn get(scrape: bool, url: &str, no_ignore: bool, no_comments: bool) -> Result<(), Box<dyn Error>> {
        if scrape {
            match Self::fetch_items(url).await {
                Ok(response) => {
                    if let Some(success) = response.success {
                        if !success {
                            ErrorsAlerts::generic(&response.message);
                            return Ok(())
                        }
                    }
    
                    if let Some(total) = response.total {
                        if total > 0 {
                            if !response.list.is_empty() {
                                for item in &response.list {
                                    if !item.encrypted {
                                        let path = "./";
                                        let url = &item.url;

                                        Download::download_pdf(
                                            url,
                                            path,
                                            no_ignore, 
                                            no_comments
                                        ).await?;
                                    }
                                }
                            }
                        } else {
                            ErrorsAlerts::generic(&response.message);
                        }
                    }
                }

                Err(e) => ErrorsAlerts::generic(&e.to_string())
            }
        }
    
        Ok(())
    }
    
}
