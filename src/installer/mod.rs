// Move to writable bin dir and add to bashrc path

use std::{env, fs::OpenOptions};
use std::io::Write;


pub fn install() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir();
    let bashrc_path = format!("/home/{}/.bashrc", env!("USER"));
    // Append alias to ~/.bashrc
    let mut bashrc = OpenOptions::new()
    .append(true)
    .open(bashrc_path)
    .unwrap();

    let fn_to_add = format!(
        "qs() {{ cd $({}/quick-switch $1); }}", current_dir.unwrap().to_str().unwrap()
    );

    writeln!(bashrc, "{}", fn_to_add)?;
    Ok(())
}