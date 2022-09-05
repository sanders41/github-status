mod github_api;
mod options;

use crate::github_api::{ComponentInfo, IncidentInfo, StatusInfo, SummaryInfo};
use crate::options::{Command, Options};

use clap::Parser;

fn main() {
    let opt = Options::parse();

    match opt.command {
        Command::AllIncidents => IncidentInfo::print_all(),
        Command::Summary => SummaryInfo::print(),
        Command::Status => StatusInfo::print(),
        Command::Component => ComponentInfo::print(),
        Command::UnresolvedIncidents => IncidentInfo::print_unresolved(),
    }
}
