use clap::Parser;
use std::error::Error;

use crate::{
    args_cli::*,
    cmd::monset::Monset,
    addons::scrape::Scrape,
    syntax::blocks::readme_block::ReadMeBlock,

    ui::{
        ui_base::UI,
        errors_alerts::ErrorsAlerts,
    },
    
    monlib::{
        pull::MonlibPull,
        push::MonlibPush,
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

    async fn options(&self, options: &str) -> Result<(), Box<dyn Error>> {
        match options {
            "open-env" => Env.open_env_file()?,
            "open-settings" => Settings.open_settings_file()?,
            "write-env" => WriteEnv::new(None, None).add_env_var()?,
            "download-env" => DownloadConfigsFiles.env_file(true, true).await?,
            "download-settings" => DownloadConfigsFiles.settings_file(true, true).await?,
            _ => (),
        };

        Ok(())
    }

    pub async fn init(&self) {
        let (print, force_mode) = (false, false);

        if let Err(err) = DownloadConfigsFiles.env_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        if let Err(err) = DownloadConfigsFiles.settings_file(print, force_mode).await {
            ErrorsAlerts::generic(&err.to_string());
        }

        let flags = Flags::parse();
        let flags_clone = flags.clone();

        if let Some(command) = flags.command {
            match command {
                Commands::Run { file } => {
                    UI::header();
                    let monset = Monset::new(&file);

                    let _ = monset.downloads(&flags_clone).await;
                    let _ = monset.run_code().await;
                    let _ = ReadMeBlock.render_block_and_save_file(&file, &flags_clone);
                },

                Commands::Pull { file } => {
                    UI::header();
                    let _ = MonlibPull.pull(&file, &flags_clone).await;
                },

                Commands::Push { file } => {
                    UI::header();
                    let _ = MonlibPush.push(&file).await;
                },

                Commands::Scrape { url } => {
                    UI::header();
                    let _ = Scrape.get(&flags_clone, &url).await;
                },

                Commands::Options { options } => {
                    let _ = self.options(&options).await;
                },

                Commands::Auth { option } => {
                    match option.as_str() {
                        "login" => println!("monlib login"),
                        "logout" => {
                            if let Err(err) = WriteEnv::new(Some("MONLIB_API_KEY".to_owned()), Some("".to_string())).edit_env_var() {
                                ErrorsAlerts::generic(&err.to_string());
                            }
                        },
                        _ => (),
                    };
                },
            }
        }
    }
    
}
