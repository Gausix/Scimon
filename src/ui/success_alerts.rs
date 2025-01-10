extern crate colored;

use colored::*;

use crate::{
    utils::domain::Domain,
    system::general::General,
};

pub struct SuccessAlerts;

impl SuccessAlerts {

    pub fn env() {
        let current_datetime = General.date_time();
        println!("{} Downloaded env file", current_datetime.green().bold());
    }

    pub fn write_env(var_name: &str) {
        let current_datetime = General.date_time();

        println!("{} Added env '{}' variable", 
            current_datetime.green().bold(),
            var_name.blue(),
        );
    }

    pub fn download(file: &str, url: &str, password: bool) {
        let mut encrypted_emoji = "";

        let domain = Domain::new(url).get();
        let current_datetime = General.date_time();
    
        if password {
            encrypted_emoji = "🔒";
        }
    
        println!(
            "{} Downloaded {} ({}) {}", 
            current_datetime.green().bold(), 
            file.blue(), 
            domain.cyan(), 
            encrypted_emoji
        );
    }
  
    pub fn download_and_generated_pdf(file: &str, url: &str) {
        let domain = Domain::new(url).get();
        let current_datetime = General.date_time();
    
        println!(
            "{} Downloaded and generated pdf {} ({})", 
            current_datetime.green().bold(), 
            file.blue(), 
            domain.cyan(),
        );
    }

    pub fn qrcode(file: &str) {
        let current_datetime = General.date_time();
    
        println!(
            "{} QR Code saved in {}", 
            current_datetime.green().bold(), 
            file.blue(), 
        );
    }

    pub fn math(file: &str) {
        let current_datetime = General.date_time();
    
        println!(
            "{} Math equation rendered and saved in {}", 
            current_datetime.green().bold(), 
            file.blue(), 
        );
    }
  
    pub fn cover_generated(file: &str) {
        let current_datetime = General.date_time();
    
        println!(
            "{} Cover saved in {}", 
            current_datetime.green().bold(), 
            file.blue(),
        );
    }

}
