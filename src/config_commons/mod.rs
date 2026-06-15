use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub working_directory: String,
    pub filename_of_repo: String,
    pub standard_logs: String,
    pub error_logs: String,
    pub bluetooth_controller_name: String,
    pub obj_id: String,
    pub auth_token: String,
    pub wifi_ssid: String,
    pub wifi_password: String,
    pub wifi_country: String,
    pub hostname: String,
    pub account_name: String,
    pub account_password: String,
}
