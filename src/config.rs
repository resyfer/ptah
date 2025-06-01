use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_semver::semver::Version;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub build: Option<BuildConfig>,
    pub version: Version,
    pub executable: Vec<ExecConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfig {
    #[serde(default = "default_build_dir")]
    pub dir: PathBuf,
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

fn default_build_dir() -> PathBuf {
    PathBuf::from("build")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionConfig {
    pub key: String,
    pub value: String,
}
