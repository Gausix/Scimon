use lopdf::Document;
use indicatif::ProgressBar;
use reqwest::header::CONTENT_TYPE as MimeType;

use std::{
    error::Error,
    path::PathBuf,
    io::BufReader,
    
    fs::{
        self,
        File
    },
};

use crate::{
    ui::ui_base::UI,
    utils::remote::Remote,
    render::render::Render,

};

pub struct Pdf;

impl Pdf {

    pub async fn is_pdf_file(&self, url: &str) -> Result<bool, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        if !response.status().is_success() {
            return Ok(false);
        }

        if let Some(content_type) = response.headers().get(MimeType) {
            if let Ok(content_type_str) = content_type.to_str() {
                if content_type_str == "application/pdf" || content_type_str == "application/octet-stream" {
                    return Ok(true);
                }
            }
        }
    
        Ok(false)
    }

    pub async fn create_pdf(&self, content: &str, path: PathBuf, url: &str) -> Result<(), Box<dyn Error>> {
        let len = Remote.get_file_size(url).await?;
        let pdf_contents = Render.connect_to_browser(content).await?;
    
        let pb = ProgressBar::new(len);

        pb.set_style(UI::pb_template());
        pb.set_message("Generating PDF...");

        fs::write(path, pdf_contents)?;
        pb.finish_with_message("Download and generated completed!");

        Ok(())
    }

    pub fn is_pdf_encrypted(&self, file_path: &str) -> bool {
        if let Ok(file) = File::open(file_path) {
            let reader = BufReader::new(file);

            if let Ok(doc) = Document::load_from(reader) {
                return doc.is_encrypted();
            }
        }
        
        false
    }

}
