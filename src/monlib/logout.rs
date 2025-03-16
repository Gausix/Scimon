use crate::{
    consts::addons::Addons,
    configs::write_env::WriteEnv,
    ui::errors_alerts::ErrorsAlerts,
};

pub struct MonlibLogout;

impl MonlibLogout {

    pub fn logout(&self) {
        if let Err(err) = WriteEnv::new(Some(Addons::MONLIB_API_ENV.to_owned()), Some("".to_string())).edit_env_var() {
            ErrorsAlerts::generic(&err.to_string());
        }
    }

}
