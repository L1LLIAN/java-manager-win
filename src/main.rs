mod args;
mod cmd;
mod config;

use clap::StructOpt;
use cmd::set;

use crate::{
    args::{Args, Commands},
    cmd::add,
    cmd::{list, remove},
    config::Config,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config: Config = confy::load("jvm-manager")?;

    let args = Args::parse();
    match &args.command {
        Some(x) => match x {
            Commands::List => list(config),
            Commands::Add { path } => add(path, &mut config),
            Commands::Remove { path } => remove(path, &mut config),
            Commands::Set { path } => set(*path, &mut config),
        },

        None => {
            println!("Current path = {:?}", config.current_path);
            Ok(())
        }
    }
}
