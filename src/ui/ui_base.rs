extern crate colored;
extern crate figlet_rs;

use colored::*;
use figlet_rs::FIGfont;
use indicatif::ProgressStyle;

use crate::{
    system::system::System,
    configs::global::Global,
};

pub struct UI;

impl UI {

    pub const PB_STYLE: &'static str = "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})";

    pub fn header() {
        let standard_font = FIGfont::standard().unwrap();

        if let Some(title) = standard_font.convert(Global::APP_NAME) {
            println!("{}", title.to_string().blue());
            println!("-------------------------------------------------------------------");
            println!("📜 Version: {}", Global::APP_VERSION.yellow());
            println!("🏠 Homepage: {} | {}", Global::APP_HOMEPAGE.blue(), Global::APP_AUTHOR.green());
            println!("⏰ Started in: {}", System::date_time().blue());
            println!("-------------------------------------------------------------------");
        }
    }
 
    pub fn pb_template() -> ProgressStyle {
        ProgressStyle::with_template(UI::PB_STYLE).unwrap().progress_chars("█░")
    }

}
