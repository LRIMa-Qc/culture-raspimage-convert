use guestfs::{AddDriveOptArgs, Handle};
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
    println!("{:?}", partitions);
    g.mount(&partitions[1], "/").unwrap();

    // read a file
    let data = g.read_file("/etc/sudoers").unwrap();
    println!("{:?}", data);

    // write a file
    // sync/umount/close
    g.sync().unwrap();
    g.umount_all().unwrap();
}
