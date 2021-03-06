use clap::{Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List added jvm paths
    List,

    /// Add a jvm path
    Add { path: String },

    /// Remove a jvm path
    Remove { path: String },

    /// Set a current jvm path (0 indexed)
    Set { path: usize },
}
