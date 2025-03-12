extern crate open;
use rpassword::prompt_password;

use std::{
    path::PathBuf,
    fs::OpenOptions,

    io::{
        self,
        Write,
        Error as IoError,
    },
};

use crate::{
    consts::folders::Folders,
    ui::success_alerts::SuccessAlerts,
};

pub struct WriteEnv {
    key: String,
    value: String,
}

impl WriteEnv {

    pub fn new(key: Option<String>, val: Option<String>) -> Self {
        let key = key.unwrap_or_else(|| {
            print!("Enter the variable name: ");
            io::stdout().flush().expect("Failed to flush buffer");

            let mut key = String::new();
            io::stdin().read_line(&mut key).expect("Failed to read variable name");

            key.trim().to_string().to_uppercase()
        });

        let value = val.unwrap_or_else(|| {
            prompt_password("Enter the variable value: ").unwrap()
        });

        Self { key, value }
    }

    pub fn add_env_var(&self) -> Result<(), IoError> {
        let app_folder = &*Folders::APP_FOLDER;
        let env_path: PathBuf = app_folder.join(".env");

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(env_path)?;

        writeln!(file, "{}=\"{}\"", self.key, self.value)?;
        SuccessAlerts::write_env(&self.key);

        Ok(())
    }
  
}
