mod args;
mod config;
mod win;

use clap::StructOpt;

use crate::{
    args::{Args, Commands},
    config::Config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config: Config = confy::load("jvm-manager")?;
    println!("config = {:?}", &config);

    let args = Args::parse();
    match &args.command {
        Commands::List => list(config)?,
        Commands::Add { path } => add(path, &mut config)?,
    }

    Ok(())
}

fn list(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let paths = config.jvm_paths;
    if paths.len() == 0 {
        println!("You have no paths added!");
    } else {
        println!(
            "You have {} path{} added:",
            paths.len(),
            if paths.len() == 1 { "" } else { "s" }
        );
        for path in paths {
            println!(" - {}", path);
        }
    }
    Ok(())
}

fn add(path: &String, config: &mut Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut final_path = path.clone();
    if !final_path.ends_with('\\') {
        final_path.push('\\');
    }

    if config.jvm_paths.contains(&final_path) {
        println!("The path {} is already added!", path);
        return Ok(());
    }

    config.jvm_paths.push(final_path);
    println!("Added path {}!", path);

    confy::store("jvm-manager", config)?;
    Ok(())
}
