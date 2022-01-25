mod args;
mod config;
mod win;

use std::str::from_utf8_mut;

use clap::StructOpt;

use crate::{
    args::{Args, Commands},
    config::Config,
    win::get_path_from_reg,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = confy::load("jvm-manager")?;
    confy::store("jvm-manager", &config)?;
    println!("config = {:?}", &config);

    let args = Args::parse();
    match &args.command {
        Commands::List => {
            list()?
        }
    }

    Ok(())
}

fn list() -> Result<(), Box<dyn std::error::Error>> {
    let mut path_vec = get_path_from_reg();
    let path_str = from_utf8_mut(&mut path_vec)?;
    println!("path_str = {}", path_str);

    Ok(())
}