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
    ActiveMaintenance {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Gets a list of all incidents.
    AllIncidents {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Gets a list of the 50 most recent scheduled maintenances.
    AllScheduledMaintenances {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Status of each component.
    Component {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Gets the current status
    Status {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Gets a summary for the current GitHub status.
    Summary {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Gets a list of any unresolved incidents.
    UnresolvedIncidents {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Gets a list of upcoming maintenance
    UpcomingMaintenance {
        #[clap(short, long, help = "If set the output will be displayed in a pager")]
        pager: bool,
    },

    /// Continue polling for status
    Watch {
        #[clap(
            short,
            long,
            default_value_t = 1,
            help = "The duration to wait between polling in minutes"
        )]
        duration: u64,

        #[clap(
            short,
            long,
            help = "Cancel the watch when all services are operational"
        )]
        cancel_when_operational: bool,
    },
}
