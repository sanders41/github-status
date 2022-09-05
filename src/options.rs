use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about = "Checks the status of GitHub")]
pub struct Options {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Gets a list of active maintenance.
    ActiveMaintenance,

    /// Gets a list of all incidents.
    AllIncidents,

    /// Gets a list of the 50 most recent scheduled maintenances.
    AllScheduledMaintenances,

    /// Status of each component.
    Component,

    /// Gets the current status
    Status,

    /// Gets a summary for the current GitHub status.
    Summary,

    /// Gets a list of any unresolved incidents.
    UnresolvedIncidents,

    /// Gets a list of upcoming maintenance
    UpcomingMaintenance,
}
