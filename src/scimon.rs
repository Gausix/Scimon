use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::Flags,
    cmd::monset::Monset,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
    },

    addons::{
        scrape::Scrape,
        monlib::Monlib,
    },

    configs::{
        env::Env,
        settings::Settings,
        write_env::WriteEnv,
        configs_files::DownloadConfigsFiles,
    },
};

pub struct Scimon;

impl Scimon {
    
    async fn options(options: &str) -> Result<(), Box<dyn Error>> {
        match options {
            "open-env" => Env::open_env_file()?,
            "write-env" => WriteEnv::new().add_env_var()?,
            "open-settings" => Settings::open_settings_file()?,
            "download-env" => DownloadConfigsFiles::env_file(true, true).await?,
            "download-settings" => DownloadConfigsFiles::settings_file(true, true).await?,
            _ => (),
        };
        
        Ok(())
    }

    pub async fn init() {
        let (print, force_mode) = (false, false);

        if let Err(err) = DownloadConfigsFiles::env_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        if let Err(err) = DownloadConfigsFiles::settings_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        let flags = Flags::parse();
        let url = flags.url.as_deref().unwrap_or_default();
        let run = flags.run.as_deref().unwrap_or_default();
        let options = flags.options.as_deref().unwrap_or_default();
        
        if !run.is_empty() {
            UI::header();
            let monset = Monset::new(run);

            if !Monlib::check_is_user(run) {
                let _ = monset.prints().await;
                let _ = monset.downloads(&flags).await;
                let _ = monset.run_code().await;
                
                let _ = ReadMeBlock::render_block_and_save_file(run, &flags);
            } else {
                let _ = Monlib::get(run, &flags).await;
            }
        }

        let _ = Scrape::get(&flags, url).await;
        let _ = Self::options(options).await;
    }

}
