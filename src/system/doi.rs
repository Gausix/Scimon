use crate::consts::uris::Uris;

pub struct DOI {
    url: String,
}

impl DOI {

    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn scihub(&self) -> String {
        let url_str = self.url.replace("https://", "").replace("http://", "").replace(Uris::PROVIDERS_DOMAINS[6], "");
        let last_slices = url_str
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        last_slices[0].to_string() + "/" + last_slices[1]
    }

    pub fn scihub_name(&self) -> (String, String) {
        let url_str = self.url.replace("https://", "").replace("http://", "").replace(Uris::PROVIDERS_DOMAINS[6], "");
        let last_slices = url_str
            .split('/')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        (self.url.clone(), last_slices[1].to_string() + ".pdf")
    }

}