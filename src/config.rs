use std::collections::HashMap;
use std::fs::File;
use std::io::{Error, Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub overrides: HashMap<String, Override>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub retries: u32,
    #[serde(default)]
    pub insults: bool,
    #[serde(default)]
    pub allow_all: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Override {
    #[serde(default)]
    pub allowed_commands: Vec<String>,
    #[serde(default)]
    pub password: bool,
    #[serde(default)]
    pub runas: Vec<String>,
    #[serde(default)]
    pub is_group: bool,
}

impl Default for Override {
    fn default() -> Self {
        Override {
            allowed_commands: Vec::new(),
            password: true,
            runas: Vec::new(),
            is_group: false,
        }
    }
}

impl Default for General {
    fn default() -> Self {
        General {
            enabled: false,
            prompt: "Password for {}:".to_string(),
            retries: 1,
            insults: false,
            allow_all: false,
        }
    }
}

pub fn get_config() -> Result<Config, Error> {
    let mut f = File::open("./config.toml").expect("Config does not exist...");
    let mut cfg = String::new();
    f.read_to_string(&mut cfg)?;
    let actual = toml::from_str::<crate::config::Config>(cfg.as_str()).expect("oof");
    Ok(actual)
}
