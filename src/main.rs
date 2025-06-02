use gumdrop::Options;

use crate::{args::Args, build::build_project, init::init_project};

mod args;
mod build;
mod config;
mod error;
mod gcc;
mod init;

fn main() {
    let opts = Args::parse_args_default_or_exit();
    println!("{:#?}", opts);

    let Args {
        command,
        help: _,
        version: _,
    } = opts;

    if let None = command {
        println!("Please mention a command.");
    }

    let command = command.unwrap();

    let res = match command {
        args::Command::Build(build_args) => build_project(build_args),
        args::Command::Init(init_args) => init_project(init_args),
    };

    println!("{:#?}", res);
}
