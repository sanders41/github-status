mod github_api;
mod options;

use std::time::Duration;

use crate::github_api::{ComponentInfo, IncidentInfo, MaintenanceInfo, StatusInfo, SummaryInfo};
use crate::options::{Command, Options};

use clap::Parser;

fn main() {
    let opt = Options::parse();

    match opt.command {
        Command::ActiveMaintenance { pager } => MaintenanceInfo::print_activate(pager),
        Command::AllIncidents { pager } => IncidentInfo::print_all(pager),
        Command::AllScheduledMaintenances { pager } => MaintenanceInfo::print_all(pager),
        Command::Component { pager } => ComponentInfo::print_info(pager),
        Command::Status { pager } => StatusInfo::print_info(pager),
        Command::Summary { pager } => SummaryInfo::print_info(pager),
        Command::UnresolvedIncidents { pager } => IncidentInfo::print_unresolved(pager),
        Command::UpcomingMaintenance { pager } => MaintenanceInfo::print_upcoming(pager),
        Command::Watch {
            duration,
            cancel_when_operational,
        } => SummaryInfo::watch(Duration::from_secs(duration * 60), cancel_when_operational),
    }
}
