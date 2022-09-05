use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about = "Checks the status of GitHub")]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Gets a list of all incidents.
    AllIncidents,

    /// Status of each component.
    Component,

    /// Get the current status
    Status,

    /// Get a summary for the current GitHub status.
    Summary,

    /// Gets a list of any unresolved incidents.
    UnresolvedIncidents,
}
