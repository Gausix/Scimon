use clap::{
    Parser, 
    Subcommand
};

#[derive(Clone)]
#[derive(Parser)]
#[command(author, version, about)]
pub struct Flags {
    #[arg(short, long, global = true)]
    /// URL to perform scraping on the page
    pub url: Option<String>,

    #[arg(long, global = true)]
    /// Select scraping mode
    pub scrape: bool,

    #[arg(long, global = true)]
    /// Ignore PDF files
    pub no_ignore: bool,

    #[arg(long, global = true)]
    /// Disable the !open_link directive
    pub no_open_link: bool,

    #[arg(long, global = true)]
    /// Disable the !readme directive
    pub no_readme: bool,

    #[arg(long, global = true)]
    /// Your settings
    pub options: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Execute a list of tasks or run a specific task
    Run {
        /// File or task to be executed
        file: String,
    },
}
