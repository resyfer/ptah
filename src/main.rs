use gumdrop::Options;
use log::{debug, warn};

use crate::{args::Args, build::build_project, init::init_project};

mod args;
mod build;
mod config;
mod error;
mod gcc;
mod init;

fn main() {
    let opts = Args::parse_args_default_or_exit();
    debug!("Argument Parsing {:#?}", opts);

    let Args {
        command,
        help: _,
        version,
    } = opts;

    if version {
        debug!("Printing version...");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if let None = command {
        warn!("No command specified.");
        println!("Please mention a command.");
    }

    let command = command.unwrap();

    let res = match command {
        args::Command::Build(build_args) => build_project(build_args),
        args::Command::Init(init_args) => init_project(init_args),
    };

    debug!("{:#?}", res);
}
