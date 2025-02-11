use is_url::is_url;

use std::{
    borrow::Cow,
    io::BufRead,
    error::Error,
};

use crate::{
    args_cli::Flags,
    configs::settings::Settings,
    generator::qr_code::GenQrCode,
    
    ui::{
        ui_base::UI,
        success_alerts::SuccessAlerts,
    },

    utils::{
        file::FileUtils,
        file_name_remote::FileNameRemote,
    },
    
    syntax::{
        vars::Vars,
        macro_handler::MacroHandler,
    },

    system::{
        pdf::Pdf,
        markdown::Markdown,
        reporting::Reporting,
    },
};

pub struct Tasks;

impl Tasks {

    pub async fn prints<R>(&self, reader: R) -> Result<(), Box<dyn Error>> where R: BufRead, {
        let contents = reader.lines().collect::<Result<Vec<_>, _>>()?.join("\n");

        for line in contents.lines() {
            Vars.get_print(&line);
        }

        Ok(())
    }

    pub async fn qr_codes(&self, contents: &str) -> Result<(), Box<dyn Error>> {
        if let Some(qrcode_path) = Vars.get_qrcode(contents) {
            UI::section_header("QR Codes", "normal");

            for line in contents.lines() {
                let url = line.trim().split_whitespace().next().unwrap_or("");

                if line.trim().starts_with("downloads {") {
                    continue;
                } else if line.trim().starts_with("}") {
                    break;
                }

                if !MacroHandler::handle_check_macro_line(&line, "ignore") {
                    if !url.is_empty() && is_url(&url) && url.starts_with("http") {
                        FileUtils.create_path(&qrcode_path);
            
                        let value = Settings.get("general.qrcode_size", "INT");
                        let qrcode_size = value.as_i64().expect("Invalid qrcode_size value. Must be an integer.") as usize;
            
                        let name = FileNameRemote::new(url).get();
                        let name_pdf = FileUtils.replace_extension(&name, "png");
                        let file_path = format!("{}{}", qrcode_path, name_pdf);
                        
                        GenQrCode::new(&url, qrcode_size).png(&file_path).unwrap();
                        SuccessAlerts::qrcode(file_path.as_str());
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn download(&self, contents: Option<&str>, url: &str, path: &str, flags: &Flags) -> Result<(), Box<dyn Error>> {
        let mut line_url = Cow::Borrowed(
            url.trim()
        );

        Reporting.check_download_errors(&line_url).await?;
        if !is_url(&line_url) { return Ok(()) }
    
        match MacroHandler::handle_ignore_macro_flag(&line_url, flags.no_ignore) {
            Ok(new_line) => line_url = Cow::Owned(new_line),
            Err(_) => return Ok(()),
        }

        if let Some(contents) = contents {
            Markdown.create(&contents, &line_url, &path).await?;
        }

        Pdf.download_line(&line_url, url, path).await?;
        Ok(())
    }

}
