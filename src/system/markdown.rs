extern crate open;

use std::{
    env,
    error::Error,
};

use pulldown_cmark::{
    html,
    Parser,
    Options,
};

use crate::{
    system::pdf::Pdf,
    configs::settings::Settings, 
    generator::file_name::FileName,
    ui::success_alerts::SuccessAlerts,
    
    render::{
        render_io::RenderIO,
        render_inject::RenderInject,
    }, 
    
    utils::{
        remote::Remote, 
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
};

pub struct Markdown;

impl Markdown {

    pub fn open_file(path: &str, no_open_link: bool) {
        if !no_open_link {
            let full_path = env::current_dir().expect(
                ""
            ).join(&path).to_str().unwrap().replace(
                "\\", "/"
            );

            let url_file = &format!(
                "file://{}", full_path
            );

            let _ = open::that(&url_file);
        }
    }

    pub fn get_filename_rendered(file: &str) -> String {
        let filename = if Settings.get("render_markdown.overwrite", "BOOLEAN") == true {
            ".html".to_string()
        } else {
            FileName::new(16, "html").gen()
        };

        RenderIO::get_file_path(file).replace(".html", &filename)
    }

    pub fn append_extras_and_render(markdown: &str) -> String {
        let parser = Parser::new_ext(&markdown, Options::all());
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        format!("<div class='markdown-content'>{}</div>", html_output)
    }

    pub async fn render(url: &str) -> Result<String, Box<dyn Error>> {
        let markdown_content = Remote::content(url).await?;
    
        let options = Options::empty();
        let parser = Parser::new_ext(&markdown_content, options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
    
        Ok(html_output)
    }

    pub async fn create(contents: &str, url: &str, path: &str) -> Result<(), Box<dyn Error>> {
        if Remote::check_content_type(&url, "text/markdown").await? || url.contains(".md") {
            let html_content = Self::render(url).await?;
            let content = RenderInject::html_content(contents, html_content).await?;
            
            let original_name = FileNameRemote::new(url).get();
            let new_filename = FileUtils.replace_extension(&original_name, "pdf");
            let output_path = FileUtils.get_output_path(&path, &new_filename);

            Pdf::create_pdf(&content, output_path, &url).await?;
            SuccessAlerts::download_and_generated_pdf(&new_filename, url);
        }

        Ok(())
    }

}
