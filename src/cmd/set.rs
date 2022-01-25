use std::{
    fs,
    os::windows::{self},
};

use crate::config::Config;

pub fn set(path_idx: usize, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let paths = &config.jvm_paths;
    if paths.len() <= path_idx {
        println!("Path index invalid! Max = {}", paths.len() - 1);
        return Ok(());
    }

    let path = paths.get(path_idx).unwrap();
    config.current_path = Some(path.to_string());
    confy::store("jvm-manager", &config)?;

    let user_dirs = directories::UserDirs::new().unwrap();
    let home = user_dirs.home_dir();
    let jdk_path = home.join(".jdk");

    if jdk_path.exists() && jdk_path.is_symlink() {
        fs::remove_dir(&jdk_path)?;
    }

    windows::fs::symlink_dir(path, jdk_path)?;

    println!("Set current java install to {}!", path);
    Ok(())
}
