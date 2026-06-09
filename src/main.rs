use std::{collections::HashMap, env, fs, path::PathBuf};

use guestfs::{AddDriveOptArgs, Handle};
use minijinja::Environment;
use serde::Deserialize;
use wpa_psk::{Passphrase, Ssid, wpa_psk};

#[derive(Deserialize)]
struct Config {
    image_path: String,
    working_directory: String,
    filename_of_repo: String,
    standard_logs: String,
    error_logs: String,
    controller_name: String,
    obj_id: String,
    auth_token: String,
    wifi_ssid: String,
    wifi_password: String,
    hostname: String,
}

fn main() {
    let config_path = env::args().nth(1).expect("missing config path argument");
    let config: Config =
        serde_json::from_str(&fs::read_to_string(PathBuf::from(config_path)).unwrap())
            .expect("invalid config json");

    let g = Handle::create().unwrap();
    g.add_drive(
        &config.image_path,
        AddDriveOptArgs {
            readonly: Some(false),
            format: None,
            iface: None,
            name: None,
            label: None,
            protocol: None,
            server: None,
            username: None,
            secret: None,
            cachemode: None,
            discard: None,
            copyonread: None,
        },
    )
    .expect("Drive had error, you are on your own.");
    g.launch().unwrap();
    let partitions = g.list_partitions().unwrap();
    g.mount(&partitions[1], "/").unwrap();

    handle_all(&config, &g).unwrap();
    // write a file
    // sync/umount/close
    g.sync().unwrap();
    g.umount_all().unwrap();
}

fn format_file_from_keys_in_template(
    template: &str,
    keys_in_template: HashMap<String, String>,
) -> String {
    let mut env = Environment::new();
    env.add_template("file", template).unwrap();
    let tmpl = env.get_template("file").unwrap();
    tmpl.render(keys_in_template).unwrap()
}
fn handle_systemd_boot_services(
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
        String::from("FILENAME"),
        entries.get("FILENAME").unwrap().clone(),
    );
    missing_keys.insert(
        String::from("STANDARD_LOGS"),
        entries.get("STANDARD_LOGS").unwrap().clone(),
    );

    missing_keys.insert(
        String::from("ERROR_LOGS"),
        entries.get("ERROR_LOGS").unwrap().clone(),
    );

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

fn handle_bluetooth_services(
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

fn handle_config_file(entries: &HashMap<String, String>, g: &Handle) -> Result<(), std::io::Error> {
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
    g.write("/tmp/config.ini", formatted_file.as_bytes())
        .unwrap();
    Ok(())
}

fn handle_hostname(entries: &HashMap<String, String>, g: &Handle) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/wpa_supplicant.conf")?;
    let mut missing_keys = HashMap::new();

    missing_keys.insert(
        String::from("HOSTNAME"),
        entries.get("HOSTNAME").unwrap().clone(),
    );

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write("/etc/hostname", formatted_file.as_bytes())
        .expect("fucked up the write to wpa_supplicant.conf");
    Ok(())
}

fn handle_wifi_configuration(
    entries: &HashMap<String, String>,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/wpa_supplicant.conf")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(
        String::from("WIFI_SSID"),
        entries.get("WIFI_SSID").unwrap().clone(),
    );
    let pre_shared_key = wpa_psk(
        &Ssid::try_from(entries.get("WIFI_SSID").unwrap().as_str()).expect("Bad SSID."),
        &Passphrase::try_from(entries.get("WIFI_PASSWORD").unwrap().as_str()).unwrap(),
    );
    missing_keys.insert(
        String::from("WIFI_PRESHAREDKEY"),
        hex::encode(pre_shared_key),
    );
    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write(
        "/etc/wpa_supplicant/wpa_supplicant.conf",
        formatted_file.as_bytes(),
    )
    .unwrap();
    Ok(())
}
fn handle_bootstrap_install_script(
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

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write(
        "/var/run/LRIMa-central/install.sh",
        formatted_file.as_bytes(),
    )
    .expect("fucked up the write to install.sh");
    Ok(())
}

fn handle_bootstrap_install_service(g: &Handle) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/LRIMa-centrale-install-runonce.service")?;

    let file_path = "/etc/systemd/system/LRIMa-centrale-install-runonce.service";

    g.write(file_path, current_file.as_bytes()).unwrap();

    g.ln_s(
        file_path,
        "/etc/systemd/system/multi-user.target.wants/LRIMa-central.service",
    )
    .expect("ln in bootstrap service done fucked up today,");
    Ok(())
}

fn handle_all(config: &Config, g: &Handle) -> Result<(), std::io::Error> {
    let mut entries = HashMap::new();
    entries.insert(
        String::from("WORKING_DIRECTORY"),
        config.working_directory.clone(),
    );
    entries.insert(String::from("STANDARD_LOGS"), config.standard_logs.clone());
    entries.insert(String::from("ERROR_LOGS"), config.error_logs.clone());
    entries.insert(
        String::from("CONTROLLER_NAME"),
        config.controller_name.clone(),
    );
    entries.insert(String::from("OBJ_ID"), config.obj_id.clone());
    entries.insert(String::from("AUTH_TOKEN"), config.auth_token.clone());
    entries.insert(String::from("WIFI_SSID"), config.wifi_ssid.clone());
    entries.insert(String::from("WIFI_PASSWORD"), config.wifi_password.clone());
    entries.insert(String::from("FILENAME"), config.filename_of_repo.clone());
    entries.insert(String::from("HOSTNAME"), config.hostname.clone());

    handle_systemd_boot_services(&entries, g)?;
    handle_bluetooth_services(&entries, g)?;
    handle_config_file(&entries, g)?;
    handle_wifi_configuration(&entries, g)?;
    handle_bootstrap_install_script(&entries, g)?;
    handle_bootstrap_install_service(g)?;
    Ok(())
}
