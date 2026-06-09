use std::{
    collections::HashMap,
    fs,
};

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

    handle_systemd_services(
        String::from("/opt/LRIMa-central"),
        String::from("/var/log/LRIMa/standard.log"),
        String::from("/var/log/LRIMa/error.log"),
        &g,
    )
    .unwrap();
    handle_bluetooth_services(String::from("CONTROLLER_NAME"), &g).unwrap();
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
    working_directory: String,
    standard_logs: String,
    error_logs: String,
    g: &Handle,
) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/LRIMa-central.service")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(String::from("WORKING_DIRECTORY"), working_directory);
    missing_keys.insert(String::from("STANDARD_LOGS"), standard_logs);
    missing_keys.insert(String::from("ERROR_LOGS"), error_logs);

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

fn handle_bluetooth_services(controller_name: String, g: &Handle) -> Result<(), std::io::Error> {
    let current_file = fs::read_to_string("config_file/bluetooth.conf")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(String::from("CONTROLLER_NAME"), controller_name);
    let formatted_file = format_file_from_keys_in_template(&current_file, missing_keys);
    g.write("/etc/bluetooth/main.conf", formatted_file.as_bytes())
        .unwrap();
    Ok(())
}
