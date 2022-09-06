mod github_api;
mod options;

use crate::github_api::{ComponentInfo, IncidentInfo, MaintenanceInfo, StatusInfo, SummaryInfo};
use crate::options::{Command, Options};

use clap::Parser;

fn main() {
    let opt = Options::parse();

    match opt.command {
        Command::ActiveMaintenance => MaintenanceInfo::print_activate(),
        Command::AllIncidents => IncidentInfo::print_all(),
        Command::AllScheduledMaintenances => MaintenanceInfo::print_all(),
        Command::Component => ComponentInfo::print_info(),
        Command::Status => StatusInfo::print_info(),
        Command::Summary => SummaryInfo::print_info(),
        Command::UnresolvedIncidents => IncidentInfo::print_unresolved(),
        Command::UpcomingMaintenance => MaintenanceInfo::print_upcoming(),
    }
}
