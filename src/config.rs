use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub daemons: HashMap<String, Daemon>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Daemon {
    pub pid: u32,
    pub exe: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_dev_null")]
    pub output: String,
}

fn default_dev_null() -> String {
    "/dev/null".to_string()
}

impl Config {
    pub fn load() -> Self {
        let config = match std::fs::read_to_string("easy_daemon_config.toml") {
            Ok(config) => config,
            Err(_) => {
                return Config {
                    daemons: HashMap::new(),
                };
            }
        };
        match toml::from_str::<Config>(&config) {
            Ok(daemon_config) => daemon_config,
            Err(_) => Config {
                daemons: HashMap::new(),
            },
        }
    }

    pub fn add(
        &mut self,
        name: String,
        exe: String,
        args: Option<Vec<String>>,
        output: Option<String>,
    ) {
        let daemon = Daemon {
            pid: 0,
            exe,
            args: args.unwrap_or_default(),
            output: output.unwrap_or(default_dev_null()),
        };
        self.daemons.insert(name, daemon);
    }

    pub fn save(&self) {
        let config = toml::to_string(self).unwrap();
        std::fs::write("easy_daemon_config.toml", config).unwrap();
    }
}
