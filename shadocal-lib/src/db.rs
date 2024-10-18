use std::{ops::Deref, path::PathBuf};

use anyhow::{anyhow, Context, Result};
use directories::ProjectDirs;
use polodb_core::{Collection, Database};

pub use polodb_core::{
    bson::{doc, Document},
    CollectionT,
};

use super::Profile;

const DB_FILENAME: &str = "shadocal.db";

pub struct Db(Database);

impl Db {
    pub fn new(path: Option<&str>) -> Result<Self> {
        let path = path
            .map(PathBuf::from)
            .or_else(data_directory)
            .ok_or_else(|| anyhow!("Failed to get config path"))?;
        std::fs::create_dir_all(&path).context(format!("{:?} could not be created", &path))?;

        Ok(Self(Database::open_path(path.join(DB_FILENAME))?))
    }

    pub fn accounts(&self) -> Collection<Profile> {
        self.collection::<Profile>("accounts")
    }
    pub fn add_account(&self, profile: &Profile) -> Result<()> {
        self.accounts().insert_one(profile)?;
        Ok(())
    }
}

impl Deref for Db {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn data_directory() -> Option<PathBuf> {
    ProjectDirs::from("", "", "shadocal").map(|p| p.data_local_dir().to_path_buf())
}
