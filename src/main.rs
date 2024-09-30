use clap::{Arg, Command};
use dialoguer::{Input, Select};
use std::process::Command as ProcessCommand;

fn main() {
    let _matches = Command::new("LV Manager")
        .version("1.0")
        .author("Alec <alecjdavidson@outlook.com>")
        .about("Manage Logical Volumes on RHEL")
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive"),
        )
        .get_matches();

    run();
}

fn run() {
    println!("Welcome to the Logical Volume Manager!");

    let options = vec![
        "Create a Logical Volume",
        "Extend a Logical Volume",
        "Reduce a Logical Volume",
        "Remove a Logical Volume",
    ];

    let selection = Select::new()
        .items(&options)
        .with_prompt("Please select an action")
        .interact()
        .unwrap();

    match selection {
        0 => create_logical_volume(),
        1 => extend_logical_volume(),
        2 => reduce_logical_volume(),
        3 => remove_logical_volume(),
        _ => println!("Invalid selection!"),
    }
}

fn create_logical_volume() {
    let volume_group: String = Input::new()
        .with_prompt("Enter the Volume Group name")
        .interact_text()
        .unwrap();

    let logical_volume: String = Input::new()
        .with_prompt("Enter the Logical Volume name")
        .interact_text()
        .unwrap();

    let size: String = Input::new()
        .with_prompt("Enter the size (e.g., 10G)")
        .interact_text()
        .unwrap();

    let script = format!(
        "lvcreate -L {} -n {} {}",
        size, logical_volume, volume_group
    );

    execute_bash_script(&script);
}

fn extend_logical_volume() {
    let logical_volume: String = Input::new()
        .with_prompt("Enter the Logical Volume name")
        .interact_text()
        .unwrap();

    let size: String = Input::new()
        .with_prompt("Enter the size to extend by (e.g., 10G)")
        .interact_text()
        .unwrap();

    let script = format!(
        "lvextend -L +{} /dev/mapper/{}",
        size, logical_volume
    );

    execute_bash_script(&script);
}

fn reduce_logical_volume() {
    let logical_volume: String = Input::new()
        .with_prompt("Enter the Logical Volume name")
        .interact_text()
        .unwrap();

    let size: String = Input::new()
        .with_prompt("Enter the size to reduce by (e.g., 10G)")
        .interact_text()
        .unwrap();

    let script = format!(
        "lvreduce -L -{} /dev/mapper/{}",
        size, logical_volume
    );

    execute_bash_script(&script);
}

fn remove_logical_volume() {
    let logical_volume: String = Input::new()
        .with_prompt("Enter the Logical Volume name")
        .interact_text()
        .unwrap();

    let script = format!("lvremove /dev/mapper/{}", logical_volume);

    execute_bash_script(&script);
}

fn execute_bash_script(script: &str) {
    println!("Executing: {}", script);
    let output = ProcessCommand::new("bash")
        .arg("-c")
        .arg(script)
        .output()
        .expect("Failed to execute the bash script");

    if output.status.success() {
        println!("Success: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!(
            "Error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
