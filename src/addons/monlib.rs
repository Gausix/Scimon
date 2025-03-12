extern crate reqwest;

use regex::Regex;
use serde_json::Value;
use serde::Deserialize;

use std::{
    fmt,
    error::Error,
    
    io::{
        Cursor,
        BufRead,
    }
};

use reqwest::{
    header,
    Client,
};

use crate::{
    args_cli::Flags,
    configs::env::Env,
    cmd::monset::Monset,
    consts::addons::Addons,
    regexp::regex_blocks::BlocksRegExp,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        panic_alerts::PanicAlerts,
        errors_alerts::ErrorsAlerts,
    },
};

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
enum ApiError {
    Message(String)
}

impl fmt::Display for ApiError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Message(msg) => write!(f, "{}", msg),
        }
    }

}

impl Error for ApiError {}

pub struct Monlib;

impl Monlib {

    fn validator(&self, content: &str) -> bool {
        if content.is_empty() {
            return false;
        }
    
        BlocksRegExp::GET_PATTERNS_MONLIB_VARS.iter().any(|pattern| {
            let re = Regex::new(pattern).expect("Error compiling regex");
            re.is_match(&content)
        })
    }

    pub async fn publish(&self, run: &str) -> Result<(), Box<dyn Error>> {
        if !&self.validator(&run) {
            PanicAlerts::monlib_invalid_lib();
            return Ok(());
        }

        println!("Monlib publish");
        Ok(())
    }

    pub async fn get(&self, run: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        let mut url = Addons::MONLIB_API_REQUEST.to_owned();
    
        url.push_str("packages/");
        url.push_str(&run);
        url.push_str("/raw");
    
        let client = Client::builder().danger_accept_invalid_certs(true).build()?;
        let response = client
            .get(&url)
            .header(
                header::AUTHORIZATION, format!(
                    "Bearer {}", Env.env_var("MONLIB_API_KEY")
                )
            )
            .send().await?;
    
        if response.status().is_success() {
            let result = String::new();
            let mut is_json = true;
            let data = response.text().await?;
    
            if let Ok(json_data) = serde_json::from_str::<Value>(&data) {
                if let Some(message) = json_data.get("message") {
                    if let Some(message_str) = message.as_str() {
                        return Ok(message_str.to_string());
                    }
                }
            } else {
                is_json = false;
            }
    
            if !is_json {
                let lines_iter = Cursor::new(&data).lines();
                let collected_lines: Result<String, _> = lines_iter.collect();

                if let Ok(validated_content) = collected_lines {
                    if !self.validator(&validated_content) {
                        PanicAlerts::monlib_invalid_lib();
                        return Ok(String::new());
                    }
                } else {
                    PanicAlerts::monlib_invalid_lib();
                    return Ok(String::new());
                }

                let monset = Monset::new(&url);

                let _ = monset.downloads(&flags).await;
                let _ = monset.run_code().await;
                let _ = ReadMeBlock.render_block_and_save_file(&url, &flags);
            }
    
            Ok(result)
        } else {
            let response_text = response.text().await?;
    
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&response_text) {
                let message = ApiError::Message(error_response.message);
                ErrorsAlerts::generic(&message.to_string());
    
                Ok(message.to_string())
            } else {
                Err(
                    ApiError::Message(
                        format!("Error: internal server error")
                    ).into()
                )
            }
        }
    }

}
