use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

use guestfs::{AddDriveOptArgs, Handle};
use minijinja::{Environment, context};
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
    // read a file
    g.ln_s("asd", "asd").unwrap();

    for i in files {
        println!("{}", i.name);
    }

    // write a file
    // sync/umount/close
    g.sync().unwrap();
    g.umount_all().unwrap();
}

fn format_file_from_keys_in_template(
    template: File,
    keys_in_template: HashMap<String, String>,
) -> String {
    let mut env = Environment::new();
    let buf_read = BufReader::new(template);
    let file_bytes = buf_read.buffer();
    let data_as_string = str::from_utf8(file_bytes).unwrap();
    env.add_template("file", data_as_string).unwrap();
    let tmpl = env.get_template("file").unwrap();
    match tmpl.render(context! {keys_in_template}) {
        Ok(val) => return val,
        Err(e) => panic!("you done fucked up cowboy: {}", e),
    }
}
fn handle_systemd_services(
    working_directory: String,
    standard_logs: String,
    error_logs: String,
    g: Handle,
) -> Result<(), std::io::Error> {
    let current_file = File::open("config_file/LRIMa-central.service")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(String::from("WORKING_DIRECTORY"), working_directory);
    missing_keys.insert(String::from("STANDARD_LOGS"), standard_logs);
    missing_keys.insert(String::from("ERROR_LOGS"), error_logs);

    let formatted_file = format_file_from_keys_in_template(current_file, missing_keys);

    g.write(
        "/etc/systemd/system/LRIMa-central.service",
        formatted_file.as_bytes(),
    )
    .unwrap();
    g.ln_s(
        "/etc/systemd/system/LRIMa-central.service",
        "/etc/systemd/system/multi-user.target.wants/LRIMa-central.service",
    )
    .unwrap();
    Ok(())
}

fn handle_bluetooth_services(controller_name: String, g: Handle) -> Result<(), std::io::Error> {
    let current_file = File::open("config_file/bluetooth.conf")?;
    let mut missing_keys = HashMap::new();
    missing_keys.insert(String::from("CONTROLLER_NAME"), controller_name);
    let formatted_file = format_file_from_keys_in_template(current_file, missing_keys);
    g.write("/etc/bluetooth/main.conf", formatted_file.as_bytes())
        .unwrap();
    Ok(())
}
