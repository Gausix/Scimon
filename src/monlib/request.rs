extern crate reqwest;

use reqwest::{
    Client, 
    Response
};

use crate::{
    configs::env::Env,
    consts::addons::Addons,
};

pub struct MonlibRequest;

impl MonlibRequest {

    pub async fn request(&self, url: &str) -> Result<Response, reqwest::Error> {
        let api_key = Env.env_var(Addons::MONLIB_API_ENV); 
        let client = Client::builder().danger_accept_invalid_certs(true).build().unwrap();
        
        let response = client
            .get(url)
            .header("API-Key", api_key)
        .send()
        .await?;

        Ok(response)
    }

}
