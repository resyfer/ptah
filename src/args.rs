use std::path::PathBuf;

use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Args {
    #[options(help = "Print help")]
    pub help: bool,

    #[options(help = "Print version")]
    pub version: bool,

    #[options(command)]
    pub command: Option<Command>,
}

#[derive(Debug, Options)]
pub enum Command {
    #[options(help = "Build project")]
    Build(BuildArgs),

    #[options(help = "Initialize project")]
    Init(InitArgs),
}

#[derive(Debug, Options, Default)]
pub struct BuildArgs {
    #[options(help = "Print help")]
    pub help: bool,

    #[options(help = "Build configuration file", default = "config.json")]
    pub config_file: PathBuf,
}

#[derive(Debug, Options, Default)]
pub struct InitArgs {
    #[options(help = "project directory", default = ".")]
    pub dir: PathBuf,
}
