mod github_api;
mod options;

use crate::github_api::{ComponentInfo, StatusInfo, SummaryInfo};
use crate::options::{Command, Options};

use clap::Parser;

fn main() {
    let opt = Options::parse();

    match opt.command {
        Command::Summary => SummaryInfo::print().unwrap(),
        Command::Status => StatusInfo::print().unwrap(),
        Command::Component => ComponentInfo::print().unwrap(),
    }
}
