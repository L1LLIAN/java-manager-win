mod args;
mod cmd;
mod config;
mod win;

use clap::StructOpt;

use crate::{
    args::{Args, Commands},
    cmd::add,
    cmd::list,
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
