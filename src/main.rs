mod api;
mod cmd;
mod utils;
mod configs;

extern crate colored;
extern crate figlet_rs;

use clap::Parser;
use std::error::Error;

use colored::*;
use figlet_rs::FIGfont;

use crate::configs::global::Global;

use crate::utils::misc::Misc;

use crate::configs::env::Env;

use crate::api::api_get_list::ApiGetList;
use crate::api::api_publish_list::*;

use crate::cmd::bootstrap::Bootstrap;

#[derive(Parser)]
#[clap(author, version, about)]
struct Args{
    #[arg(long)]
    /// The paimon file to create a new Monlib list
    file: Option<String>,

    #[arg(long)]
    /// Title of a new Monlib list
    title: Option<String>,

    #[arg(long)]
    /// Privacy of a new Monlib list
    privacy: Option<String>,

    #[arg(short, long)]
    /// Run a Monlib list or execute a specific list
    run: Option<String>,

    #[arg(long)]
    /// No ignore any pdf files
    noignore: bool,

    #[arg(long)]
    /// Disable the comments and !debug macro
    no_comments: bool,

    #[arg(long)]
    /// Your Kindle email for send the ebooks for your account
    kindle: Option<String>,

    #[arg(long)]
    /// Your Paimon settings
    options: Option<String>,

    #[arg(long)]
    /// Inspect the pdf files at library selected
    inspect: bool,

    #[arg(short, long)]
    /// Publish a new library in your Monlib account
    publish: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = Env::download_env_file(false).await {
        eprintln!("Error: {}", err);
    }

    let args_parser: Args = Args::parse();
    let run = args_parser.run.as_deref().unwrap_or_default();

    if !run.is_empty() {
        let standard_font = FIGfont::standard().unwrap();
        
        if let Some(title) = standard_font.convert(Global::APP_NAME) {
            println!("{}", title.to_string().blue());
            println!("-------------------------------------------------------------------");
            println!("📜 Version: {}", Global::APP_VERSION.yellow());
            println!("🏠 Homepage: {} | {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
            println!("⏰ Started in: {}", Misc::date_time().blue());
            println!("-------------------------------------------------------------------");
        }
        
        if !Misc::check_format(run) {
            let _ = Bootstrap::read_paimon_file(
                run, args_parser.noignore, args_parser.no_comments, args_parser.kindle
            ).await;
        } else {
            ApiGetList::get(
                run, args_parser.noignore, args_parser.no_comments, args_parser.kindle
            ).await?;
        }
    }

    if args_parser.publish {
        if let (Some(file_path), Some(title)) = (&args_parser.file, &args_parser.title) {
            let privacy = args_parser.privacy.clone();

            let _ = api_publish_list(
                file_path, title, privacy.as_deref()
            ).await;
        } else {
            eprintln!("Error: Both 'file' and 'title' are required for publishing a library.");
        }
    }
    
    let options = &args_parser.options.as_deref().unwrap_or_default();
    Env::options_parser(options).await?;

    Ok(())
}
