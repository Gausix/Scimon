extern crate reqwest;

use serde_json::{
    Value,
    from_str,
};

use std::{
    error::Error,
    
    io::{
        Cursor,
        BufRead,
    }
};

use reqwest::Client;

use crate::{
    args_cli::Flags,
    configs::env::Env,
    cmd::monset::Monset,
    consts::addons::Addons,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        panic_alerts::PanicAlerts,
        errors_alerts::ErrorsAlerts,
    },

    handlers::{
        monlib_errors::*,
        monlib_handlers::MonlibHandlers,
    },
};

pub struct MonlibPull;

impl MonlibPull {

    pub async fn pull(&self, run: &str, flags: &Flags) -> Result<String, Box<dyn Error>> {
        let mut url = Addons::MONLIB_API_REQUEST.to_owned();
        let api_key = Env.env_var(Addons::MONLIB_API_ENV);
    
        url.push_str("packages/");
        url.push_str(&run);
        url.push_str("/raw");
    
        let client = Client::builder().danger_accept_invalid_certs(true).build()?;
        let response = client
            .get(&url)
            .header("API-Key", api_key) // <--- Aqui está a correção
            .send()
            .await?;
    
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
                    if !MonlibHandlers.validator_lib(&validated_content) {
                        PanicAlerts::monlib_invalid_lib();
                        return Ok(String::new());
                    }
                } else {
                    PanicAlerts::monlib_invalid_lib();
                    return Ok(String::new());
                }

                let monset = Monset::new(&data);

                let _ = monset.downloads_raw(&flags).await?;
                let _ = monset.run_code_raw().await;
                let _ = ReadMeBlock.render_block_and_save_file(&url, &flags);
            }
    
            Ok(result)
        } else {
            let response_text = response.text().await?;
    
            if let Ok(error_response) = from_str::<ErrorResponse>(&response_text) {
                let message = ApiError::Message(error_response.error);
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
