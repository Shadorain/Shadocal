use std::{collections::HashMap, fs::write, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use directories::ProjectDirs;

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub calendars: HashMap<String, String>,

    #[serde(skip)]
    path: PathBuf,
}

impl Config {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let path = path
            .map(PathBuf::from)
            .or_else(data_directory)
            .ok_or_else(|| anyhow!("Failed to get config path"))?
            .join(CONFIG_FILENAME);
        if !path.exists() {
            std::fs::write(&path, toml::to_string(&Config::default())?)?;
        }

        let mut config = toml::from_str::<Config>(&std::fs::read_to_string(&path)?)?;
        config.path = path;
        Ok(config)
    }

    pub fn add_calendar(&mut self, id: String, token: String) -> Result<()> {
        self.calendars.insert(id, token);
        self.write()
    }
    pub fn write(&self) -> Result<()> {
        Ok(write(
            self.path.as_os_str(),
            toml::to_string_pretty(&self)?,
        )?)
    }
}

fn data_directory() -> Option<PathBuf> {
    ProjectDirs::from("", "", "shadocal").map(|p| p.data_local_dir().to_path_buf())
}
