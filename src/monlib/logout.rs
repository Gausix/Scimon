use crate::{
    configs::write_env::WriteEnv,
    ui::errors_alerts::ErrorsAlerts,
};

pub struct MonlibLogout;

impl MonlibLogout {

    pub fn logout(&self) {
        if let Err(err) = WriteEnv::new(Some("MONLIB_API_KEY".to_owned()), Some("".to_string())).edit_env_var() {
            ErrorsAlerts::generic(&err.to_string());
        }
    }

}
