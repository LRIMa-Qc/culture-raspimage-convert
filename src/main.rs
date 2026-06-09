use std::{collections::HashMap, fs};

use guestfs::{AddDriveOptArgs, Handle};
use minijinja::Environment;
fn main() {
    let g = Handle::create().unwrap();
    g.add_drive(
        "2026-04-21-raspios-trixie-arm64-lite.img",
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

    handle_all(&g).unwrap();
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
fn handle_systemd_services(
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
    .unwrap();
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

fn handle_config_file(
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
    g.write("/opt/LRIMa-central/config.ini", formatted_file.as_bytes())
        .unwrap();
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
    missing_keys.insert(
        String::from("WIFI_PRESHAREDKEY"),
        entries.get("WIFI_PRESHAREDKEY").unwrap().clone(),
    );

    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write("/etc/wpa_supplicant/wpa_supplicant.conf", formatted_file.as_bytes())
        .unwrap();
    Ok(())
}
fn handle_all(g: &Handle) -> Result<(), std::io::Error> {
    let mut entries = HashMap::new();
    entries.insert(
        String::from("WORKING_DIRECTORY"),
        String::from("/opt/LRIMa-central"),
    );
    entries.insert(
        String::from("STANDARD_LOGS"),
        String::from("/var/log/LRIMa/standard.log"),
    );
    entries.insert(
        String::from("ERROR_LOGS"),
        String::from("/var/log/LRIMa/error.log"),
    );
    entries.insert(String::from("CONTROLLER_NAME"), String::from("CONTROLLER_NAME"));
    entries.insert(String::from("OBJ_ID"), String::from("OBJ_ID"));
    entries.insert(String::from("AUTH_TOKEN"), String::from("AUTH_TOKEN"));
    entries.insert(String::from("WIFI_SSID"), String::from("WIFI_SSID"));
    entries.insert(
        String::from("WIFI_PRESHAREDKEY"),
        String::from("WIFI_PRESHAREDKEY"),
    );

    handle_systemd_services(&entries, g)?;
    handle_bluetooth_services(&entries, g)?;
    handle_config_file(&entries, g)?;
    handle_wifi_configuration(&entries, g)?;
    Ok(())
}
