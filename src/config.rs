use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Config {
    V1(ConfigV1),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigV1 {
    pub version: String,
    pub blog: Blog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blog {
    pub site_root: String,
    #[serde(default)]
    pub index_page: IndexPage,
    #[serde(default)]
    pub year_page: YearPage,
    #[serde(default)]
    pub tag_page: TagPage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct IndexPage {
    #[serde(default = "ten")]
    pub entries_per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct YearPage {
    #[serde(default = "fifteen")]
    pub entries_per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TagPage {
    #[serde(default = "fifteen")]
    pub entries_per_page: u32,
}

impl Config {
    pub fn load(src_dir: &PathBuf) -> Result<Self, Error> {
        let config_path = if src_dir.join("salmon.yml").exists() {
            src_dir.join("salmon.yml")
        } else if src_dir.join("salmon.yaml").exists() {
            src_dir.join("salmon.yaml")
        } else {
            return Err(failure::format_err!(
                "Config file not found.\nPut salmon.yaml to {:?}",
                src_dir
            ));
        };

        let mut file = File::open(config_path)?;
        let mut config_string = String::new();
        file.read_to_string(&mut config_string)?;

        let raw_config: serde_yaml::Mapping = serde_yaml::from_str(&config_string)?;
        let version = match raw_config.get(&serde_yaml::Value::String("version".to_string())) {
            Some(v) => match v.as_str() {
                Some(vv) => vv,
                None => return Err(failure::format_err!("`version` is not string.")),
            },
            None => return Err(failure::format_err!("Config has no version specification.")),
        };
        match version {
            "1" => {
                let config_v1: ConfigV1 = serde_yaml::from_str(&config_string)?;
                log::debug!("Completed to load config\n{:?}", config_v1);
                return Ok(Config::V1(config_v1));
            }
            _ => {
                return Err(failure::format_err!(
                    "Version `{}` is not suppored.",
                    version
                ))
            }
        };
    }
}

impl Default for IndexPage {
    fn default() -> Self {
        Self {
            entries_per_page: ten(),
        }
    }
}

impl Default for YearPage {
    fn default() -> Self {
        Self {
            entries_per_page: fifteen(),
        }
    }
}
impl Default for TagPage {
    fn default() -> Self {
        Self {
            entries_per_page: fifteen(),
        }
    }
}

fn ten() -> u32 {
    10
}

fn fifteen() -> u32 {
    15
}
