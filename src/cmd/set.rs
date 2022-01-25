use std::str::from_utf8_mut;

use crate::{
    config::Config,
    win::{get_path_from_reg, set_java_home_in_reg, set_path_in_reg},
};

pub fn set(path_idx: usize, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let paths = &config.jvm_paths;
    if paths.len() <= path_idx {
        println!("Path index invalid! Max = {}", paths.len() - 1);
        return Ok(());
    }

    let mut win_path = get_path_from_reg();
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

    set_path_in_reg(win_path + path + "\\bin;");
    set_java_home_in_reg(path.to_string());

    println!("Set current java install to {}!", path);
    Ok(())
}
