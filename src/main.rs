mod github_api;
mod options;

use crate::github_api::Summary;
use crate::options::{Command, Options};

use clap::Parser;

fn main() {
    let opt = Options::parse();

    match opt.command {
        Command::Summary => Summary::print_summary().unwrap(),
    }
}
