use crate::format_file_from_keys_in_template;
use guestfs::Handle;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub fn handle_systemd_boot_services(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/LRIMa-central.service")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("WORKING_DIRECTORY"),
        entries.get("WORKING_DIRECTORY").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("ACCOUNT_NAME"),
        entries.get("ACCOUNT_NAME").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("ACCOUNT_PASSWORD"),
        entries.get("ACCOUNT_PASSWORD").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("FILENAME"),
        entries.get("FILENAME").unwrap().clone(),
    );

    let standard = PathBuf::from(entries.get("STANDARD_LOGS").unwrap().clone());
    let error = PathBuf::from(entries.get("ERROR_LOGS").unwrap().clone());
    if standard.parent() != error.parent() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidFilename,
            "standard logs and error logs have different parent. not having them together will be a pain for all involved",
        ));
    }
    let parent = PathBuf::from(standard.parent().unwrap());
    missing_keys.insert(
        String::from("STANDARD_LOGS"),
        entries.get("STANDARD_LOGS").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("ERROR_LOGS"),
        entries.get("ERROR_LOGS").unwrap().clone(),
    );

    g.mkdir_p(parent.to_str().unwrap())
        .expect("parent couldn't be created for systemd service");
    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    let file_path = "/etc/systemd/system/LRIMa-central.service";
    g.write(file_path, formatted_file.as_bytes()).unwrap();
    g.ln_s(
        file_path,
        "/etc/systemd/system/multi-user.target.wants/LRIMa-central.service",
    )
    .expect("symlink central write failed");
    Ok(())
}

pub fn handle_bluetooth_services(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/bluetooth.conf")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("CONTROLLER_NAME"),
        entries.get("CONTROLLER_NAME").unwrap().clone(),
    );
    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write("/etc/bluetooth/main.conf", formatted_file.as_bytes())
        .unwrap();
    Ok(())
}

pub fn handle_config_file(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/config.ini")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("OBJ_ID"),
        entries.get("OBJ_ID").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("AUTH_TOKEN"),
        entries.get("AUTH_TOKEN").unwrap().clone(),
    );

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.mkdir_p("/var/local/LRIMa-central").unwrap();
    g.write(
        "/var/local/LRIMa-central/config.ini",
        formatted_file.as_bytes(),
    )
    .unwrap();
    Ok(())
}

pub fn handle_hostname(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/hostname")?;
    let mut missing_keys = HashMap::new();

    let hostname = entries.get("HOSTNAME").unwrap().clone();
    missing_keys.insert(String::from("HOSTNAME"), hostname);

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write("/etc/hostname", formatted_file.as_bytes())
        .expect("fucked up the write to hostname.conf");
    Ok(())
}

pub fn handle_wifi_configuration(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/networkmanager.nmconnection")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("WIFI_SSID"),
        entries.get("WIFI_SSID").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("WIFI_PASSWORD"),
        entries.get("WIFI_PASSWORD").unwrap().clone(),
    );
    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.mkdir_p("/etc/NetworkManager/system-connections").unwrap();
    g.write(
        "/etc/NetworkManager/system-connections/LRIMa.nmconnection",
        formatted_file.as_bytes(),
    )
    .unwrap();
    g.chmod(
        0o600,
        "/etc/NetworkManager/system-connections/LRIMa.nmconnection",
    )
    .unwrap();
    Ok(())
}

pub fn handle_wifi_country(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/cfg80211.conf")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("WIFI_COUNTRY"),
        entries.get("WIFI_COUNTRY").unwrap().clone(),
    );

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write("/etc/modprobe.d/cfg80211.conf", formatted_file.as_bytes())
        .unwrap();
    Ok(())
}
pub fn handle_bootstrap_install_script(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/install.sh")?;
    let mut missing_keys = HashMap::new();

    missing_keys.insert(
        String::from("FILENAME"),
        entries.get("FILENAME").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("WORKING_DIRECTORY"),
        entries.get("WORKING_DIRECTORY").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("ACCOUNT_NAME"),
        entries.get("ACCOUNT_NAME").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("ACCOUNT_PASSWORD"),
        entries.get("ACCOUNT_PASSWORD").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("WIFI_COUNTRY"),
        entries.get("WIFI_COUNTRY").unwrap().clone(),
    );

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.mkdir_p("/var/local/LRIMa-central").unwrap();
    g.write(
        "/var/local/LRIMa-central/install.sh",
        formatted_file.as_bytes(),
    )
    .expect("fucked up the write to install.sh");

    g.chmod(0o700, "/var/local/LRIMa-central/install.sh")
        .unwrap();
    Ok(())
}

pub fn handle_pi_camera_setup(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/pi_install_camera.sh")?;

    g.mkdir_p("/var/local/LRIMa-central").unwrap();
    g.write(
        "/var/local/LRIMa-central/pi_install_camera.sh",
        current_file.as_bytes(),
    )
    .expect("fucked up the write to pi_install_camera.sh");

    g.chmod(0o700, "/var/local/LRIMa-central/pi_install_camera.sh")
        .unwrap();

    let service_file = fs::read_to_string("config_file/camerad.service")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("WORKING_DIRECTORY"),
        entries.get("WORKING_DIRECTORY").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("FILENAME"),
        entries.get("FILENAME").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("ACCOUNT_NAME"),
        entries.get("ACCOUNT_NAME").unwrap().clone(),
    );
    let formatted_service = format_file_from_keys_in_template(&service_file, missing_keys);
    g.write(
        "/var/local/LRIMa-central/camerad.service",
        formatted_service.as_bytes(),
    )
    .expect("fucked up the write to camerad.service");
    Ok(())
}

pub fn handle_bootstrap_install_service(g: &Handle) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/LRIMa-centrale-install-runonce.service")?;

    let file_path = "/etc/systemd/system/LRIMa-centrale-install-runonce.service";

    g.write(file_path, current_file.as_bytes()).unwrap();

    g.ln_s(
        file_path,
        "/etc/systemd/system/cloud-init.target.wants/LRIMa-centrale-install-runonce.service",
    )
    .expect("ln in bootstrap service done fucked up today,");
    Ok(())
}
pub fn handle_poppup_raspos(g: &Handle) {
    g.rm("/usr/lib/systemd/system/userconfig.service").unwrap();
}

pub fn handle_sudoers_deploy(g: &Handle) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/sudoers_deploy")?;

    g.write("/etc/sudoers.d/deployer", current_file.as_bytes())
        .expect("fucked up the write to sudoers.d/deployer");
    Ok(())
}
