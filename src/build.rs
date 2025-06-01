use std::{fs::read_to_string, path::PathBuf};

use crate::{args::BuildArgs, config::Config, error::Error};

pub fn build_project(build_args: BuildArgs) -> Result<(), Error> {
    let BuildArgs {
        config_file,
        help: _,
    } = build_args;
    let config = parse_config(config_file)?;

    println!("{:#?}", config);
    todo!()
}

fn parse_config(path_str: String) -> Result<Config, Error> {
    let json_contents = read_json_file(path_str)?;
    match serde_json::from_str(&json_contents) {
        Err(e) => Err(Error::Json(e)),
        Ok(val) => Ok(val),
    }
}

fn read_json_file(path_str: String) -> Result<String, Error> {
    let path = PathBuf::from(path_str);
    match read_to_string(path) {
        Err(e) => Err(Error::Io(e)),
        Ok(val) => Ok(val),
    }
}
