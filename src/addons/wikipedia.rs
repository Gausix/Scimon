use crate::{
    consts::uris::Uris,
    
    utils::{
        url::UrlMisc,
        domain::Domain,
    },
};

pub struct Wikipedia {
    pub url: String,
}

impl Wikipedia {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn wikipedia(&self) -> (String, String) {
        let wiki_name = UrlMisc::get_last_part(&self.url);
        let wikipedia_region = format!("{}.", Domain::new(&self.url).subdomain());

        let request_url = format!("{}{}", Uris::WIKIPEDIA_API_REQUEST_PDF.to_string().replace(
            "en.", &wikipedia_region
        ), wiki_name);

        let filename = format!("{}.pdf", wiki_name);

        (request_url, filename)
    }

    pub fn wikisource(&self) -> (String, String) {
        let wiki_name = UrlMisc::get_last_part(&self.url);
        let wikipedia_region = format!("{}.", Domain::new(&self.url).subdomain());

        let request_url = format!("{}{}", Uris::WIKISOURCE_API_REQUEST_PDF.to_string().replace(
            "en.", &wikipedia_region
        ), wiki_name);

        let filename = format!("{}.pdf", wiki_name);

        (request_url, filename)
    }

}
