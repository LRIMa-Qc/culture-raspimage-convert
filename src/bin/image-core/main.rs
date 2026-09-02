use std::{collections::HashMap, fs, path::PathBuf};

mod handle;
mod template_formatting;
mod validation;

use clap::Parser;

use culture_raspimage_convert::config_commons::Config;
use guestfs::{AddDriveOptArgs, Handle};
use handle::*;
use template_formatting::format_file_from_keys_in_template;
use validation::validate_all;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    config: Group,

    #[arg(short, long, help = "Raspberry pi image path")]
    raspberry_pi_image_file_path: String,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct Group {
    /// Argument1.
    #[clap(long)]
    config_file_path: Option<String>,

    /// Argument2.
    #[clap(long)]
    config_json: Option<String>,
}

fn main() {
    let arg = Args::parse();
    let config: Config;
    if arg.config.config_file_path.is_some() {
        config = serde_json::from_str(
            &fs::read_to_string(PathBuf::from(arg.config.config_file_path.unwrap())).unwrap(),
        )
        .expect("invalid config json");
    } else {
        config = serde_json::from_str(&arg.config.config_json.unwrap().as_str()).unwrap();
    }

    let g = Handle::create().unwrap();
    g.add_drive(
        &arg.raspberry_pi_image_file_path,
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

fn handle_all(config: &Config, g: &Handle) -> Result<(), std::io::Error> {
    validate_all(config).expect("Validation of config file failed");
    let mut entries = HashMap::new();
    entries.insert(
        String::from("WORKING_DIRECTORY"),
        config.working_directory.clone(),
    );
    entries.insert(String::from("STANDARD_LOGS"), config.standard_logs.clone());
    entries.insert(String::from("ERROR_LOGS"), config.error_logs.clone());
    entries.insert(
        String::from("CONTROLLER_NAME"),
        config.bluetooth_controller_name.clone(),
    );
    entries.insert(String::from("OBJ_ID"), config.obj_id.clone());
    entries.insert(String::from("AUTH_TOKEN"), config.auth_token.clone());
    entries.insert(String::from("WIFI_SSID"), config.wifi_ssid.clone());
    entries.insert(String::from("WIFI_PASSWORD"), config.wifi_password.clone());
    entries.insert(String::from("WIFI_COUNTRY"), config.wifi_country.clone());
    entries.insert(String::from("FILENAME"), config.filename_of_repo.clone());
    entries.insert(String::from("HOSTNAME"), config.hostname.clone());
    entries.insert(String::from("ACCOUNT_NAME"), config.account_name.clone());
    entries.insert(
        String::from("ACCOUNT_PASSWORD"),
        config.account_password.clone(),
    );

    handle_systemd_boot_services(&entries, g)?;
    handle_bluetooth_services(&entries, g)?;
    handle_config_file(&entries, g)?;
    handle_wifi_configuration(&entries, g)?;
    handle_wifi_country(&entries, g)?;
    handle_bootstrap_install_script(&entries, g)?;
    handle_pi_camera_setup(&entries, g)?;
    handle_hostname(&entries, g)?;
    handle_bootstrap_install_service(g)?;
    handle_poppup_raspos(g);
    handle_sudoers_deploy(g)?;
    Ok(())
}
