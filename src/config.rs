use serenity::all::{RoleId, UserId};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use eros::IntoUnion;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub permissions: Permissions,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct General {
    pub prefix: String,
}

impl Default for General {
    fn default() -> General {
        return General {
            prefix: "~".into(),
        };
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Permissions {
    pub users: Vec<UserId>,
    pub roles: Vec<RoleId>,
}

type LoadErrors = (io::Error, toml::de::Error);
type SaveErrors = (io::Error, toml::ser::Error);

impl Config {
    pub fn load(path: impl AsRef<Path>) -> eros::Result<Config, LoadErrors> {
        let contents = fs::read_to_string(path).union()?;
        Ok(toml::from_str(&contents).union()?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> eros::Result<(), SaveErrors> {
        let contents = toml::to_string_pretty(self).union()?;
        fs::write(path, contents).union()
    }
}
