pub struct Addons;

impl Addons {

    pub const DOWNLOAD_FILES_URI: &'static str = "https://raw.githubusercontent.com/Gausix/Scimon/main/";
    pub const DEFAULT_CSS_STYLE: &'static str = "https://addons.scimon.dev/static/md-default.css";

    // Scimon
    pub const MONLIB_API_REQUEST: &'static str = "http://localhost/";
    pub const SCIMON_SCRAPE_API_ENDPOINT: &'static str = "https://addons.scimon.dev/pdfscrape?url=";
    pub const SCIMON_URLFILTER_API_ENDPOINT: &'static str = "https://addons.scimon.dev/urlfilter?url=";

    pub const README_TEMPLATE_LINK: &'static str = "https://readme.scimon.dev/";

    // Chat GPT Content Class
    pub const CHATGPT_CONTENT_CLASS: &'static str = ".markdown.prose.w-full.break-words.dark\\:prose-invert.dark";
    
}
