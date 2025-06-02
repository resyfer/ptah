use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_semver::semver::Version;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub compiler: String,

    #[serde(default)]
    pub build: BuildConfig,

    pub version: Version,
    pub executables: Vec<ExecConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfig {
    pub dir: PathBuf,
}

impl Default for BuildConfig {
    fn default() -> Self {
        BuildConfig {
            dir: PathBuf::from("build"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecConfig {
    pub name: String,
    pub src: Vec<PathBuf>,
    pub include: Option<Vec<PathBuf>>,
    pub flags: Option<Vec<String>>,
    pub options: Option<Vec<OptionConfig>>,
    pub packages: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionConfig {
    pub key: String,
    pub value: String,
}
