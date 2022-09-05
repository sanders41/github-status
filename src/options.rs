use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about = "Checks the status of GitHub")]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get a summary for the current GitHub status.
    Summary,

    /// Get the current status
    Status,

    /// Status of each component.
    Component,
}
