use culture_raspimage_convert::config_commons::Config;
use regex::Regex;
use std::{self, path::PathBuf};

#[derive(Debug)]
pub struct InvalidEntryError {
    msg: String,
}
impl InvalidEntryError {
    fn new<T: Into<String>>(msg: T) -> InvalidEntryError {
        InvalidEntryError { msg: msg.into() }
    }
}

impl std::fmt::Display for InvalidEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for InvalidEntryError {}
fn validate_unix(name: &String, source: &str) -> Result<(), InvalidEntryError> {
    let reg = Regex::new("^[a-zA-Z][-a-z0-9A-Z]*").unwrap();
    let option_match_val = reg.find(name.as_str());
    if option_match_val.is_none() {
        dbg!(option_match_val);
        return Err(InvalidEntryError::new(format!(
            "{} is not correct! please ensure you only keep lowercase and uppercase char, dashes and numbers!",
            source
        )));
    };
    let match_val = option_match_val.unwrap().as_str();
    if name.len() != match_val.len() {
        dbg!(match_val);
        return Err(InvalidEntryError::new(format!(
            "{} is not correct! please ensure you only keep lowercase and uppercase char, dashes and numbers!",
            source
        )));
    };

    Ok(())
}

fn validate_username(name: &String) -> Result<(), InvalidEntryError> {
    validate_unix(name, "username")
}

fn validate_hostname(name: &String) -> Result<(), InvalidEntryError> {
    validate_unix(name, "hostname")
}

fn validate_controller_name(name: &String) -> Result<(), InvalidEntryError> {
    validate_unix(name, "bluetooth controller name")
}

fn validate_filename(name: &String) -> Result<(), InvalidEntryError> {
    validate_unix(name, "filename of the repository")
}

fn validate_path(path: &String, source: &str) -> Result<(), InvalidEntryError> {
    let _ = match PathBuf::try_from(path) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(InvalidEntryError::new(format!(
                "{}: {} is not correct! It is currently not a valid path. Please correct and try again!",
                e, source
            )));
        }
    };
}

fn validate_standard_logs(path: &String) -> Result<(), InvalidEntryError> {
    validate_path(path, "Standard logs")
}

fn validate_error_logs(path: &String) -> Result<(), InvalidEntryError> {
    validate_path(path, "Error logs")
}

fn validate_wifi(ssid: &String, password: &String) -> Result<(), InvalidEntryError> {
    let validated_ssid = match wpa_psk::Ssid::try_from(ssid.as_str()) {
        Ok(v) => v,
        Err(e) => {
            return Err(InvalidEntryError::new(format!(
                "{}: the wifi SSID format is not correct! Please correct and try again!",
                e
            )));
        }
    };

    let validated_pass = match wpa_psk::Passphrase::try_from(password.as_str()) {
        Ok(v) => v,
        Err(e) => {
            return Err(InvalidEntryError::new(format!(
                "{}: the password format is not correct! Please correct and try again!",
                e
            )));
        }
    };

    let _ = wpa_psk::wpa_psk(&validated_ssid, &validated_pass);

    Ok(())
}
fn validate_country(country: &String) -> Result<(), InvalidEntryError> {
    if country.len() != 2 {
        return Err(InvalidEntryError::new(
            "The country code is invalid. Please make it only two characters",
        ));
    }
    if country.to_uppercase() != country.to_owned() {
        return Err(InvalidEntryError::new(
            "The country code is invalid. Please make it only uppercase characters",
        ));
    }
    Ok(())
}

pub fn validate_all(config: &Config) -> Result<(), InvalidEntryError> {
    validate_country(&config.wifi_country)?;
    validate_wifi(&config.wifi_ssid, &config.wifi_password)?;
    validate_controller_name(&config.bluetooth_controller_name)?;
    validate_error_logs(&config.error_logs)?;
    validate_standard_logs(&config.standard_logs)?;
    validate_filename(&config.filename_of_repo)?;
    validate_hostname(&config.hostname)?;
    validate_username(&config.account_name)?;

    Ok(())
}
