use std::{str::from_utf8_mut, ops::Add};

use crate::{config::Config, win::{self, set_path_in_reg}};

pub fn set(path_idx: usize, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let paths = &config.jvm_paths;
    if paths.len() <= path_idx {
        println!("Path index invalid! Max = {}", paths.len() - 1);
        return Ok(());
    }

    let mut win_path = win::get_path_from_reg();
    win_path.resize(win_path.len() - 1, 0);
    let win_path = from_utf8_mut(&mut win_path)?;
    let mut win_path = win_path.to_string();

    let old_path = &config.current_path;
    if let Some(x) = old_path {
        let x = x.to_owned() + "\\bin";
        let from = x + ";";
        let new_path = &mut win_path.replace(&from, "");
        win_path = new_path.to_string();
    }

    let path = paths.get(path_idx).unwrap();
    config.current_path = Some(path.to_string());
    confy::store("jvm-manager", &config)?;

    let mut new_path = win_path + path + "\\bin;";
    println!("new_path = {}", new_path);
    set_path_in_reg(&mut new_path);

    println!("Set current path to {}!", path);
    Ok(())
}
