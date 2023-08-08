use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

use log::info;

use crate::utils::project_root_path;
use anyhow::Result;

use super::{command::Command, global::Global, service::Service};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub global: Option<Global>,
    #[serde(default)]
    include: Vec<String>,

    #[serde(default)]
    pub services: HashMap<String, Service>,
    #[serde(default)]
    pub commands: HashMap<String, Command>,
}

impl Config {
    pub fn from_project_root() -> Config {
        let mut pr = project_root_path().expect("Program should be ran from git repository");
        pr.push("optimus.yaml");

        let c = Config::load_from_path(pr);
        match c {
            Ok(c) => c,
            Err(e) => panic!("{}", e),
        }
    }

    fn load_from_path(path: PathBuf) -> Result<Config> {
        info!("Path: {:?}", path);
        let file_string = fs::read_to_string(path.clone())?;
        let mut c = serde_yaml::from_str::<Config>(&file_string)?;
        for inner_config_path in c.include.clone() {
            let mut as_path_buf = path.clone();
            as_path_buf.push(inner_config_path);
            let inner_config = match Config::load_from_path(as_path_buf) {
                Ok(c) => c,
                Err(e) => {
                    info!("{}", e);
                    continue;
                }
            };
            c.merge_with(inner_config);
        }
        Ok(c)
    }

    fn print(self) {
        println!("{:#?}", self);
    }

    fn merge_with(&mut self, another_config: Config) {
        // self.include
        for (k, s) in another_config.services {
            self.services.insert(k, s);
        }
        for (k, c) in another_config.commands {
            self.commands.insert(k, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn configs_can_be_merged() {
        let mut config1 = Config {
            ..Default::default()
        };
        let config2 = Config {
            ..Default::default()
        };

        config1.merge_with(config2);
    }

    #[test]
    fn can_load_config_from_project_root() {
        let config = Config::from_project_root();

        assert!(config.services.contains_key("optimus"));
        assert!(config.commands.contains_key("lscmd"));
        assert!(config.commands.contains_key("start"));
    }

    #[test]
    fn config_can_loaded_with_global_and_include() {
        let config_string = "
global:
  shell: nu
include:
  - ./config
  - ./commands_optimus.yml
        ";
        let config = serde_yaml::from_str::<Config>(&config_string);
        assert!(config.is_ok());
        let config = config.unwrap();
        // assert!(config.include.is_empty());
        assert!(!config.include.is_empty());
        assert!(config.include.contains(&"./config".to_string()));
        assert!(config
            .include
            .contains(&"./commands_optimus.yml".to_string()));
    }

    #[test]
    fn config_can_loaded_with_services() {
        let config_string = r##"
services:
  optimus:
    root: .
    test: |
      cargo nextest
"##;
        let config = serde_yaml::from_str::<Config>(&config_string);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.services.get("optimus").is_some());
    }

    #[test]
    fn config_can_loaded_with_commands() {
        let config_string = r##"
commands:
    start: 
        description: |
          Start the application
        run: |
          echo "sup"
          docker compose -f compose.yml -f compose.dev.yml up -d  
    stringCmd: |
        echo "elo"
        "##;
        let config = serde_yaml::from_str::<Config>(&config_string);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!(config.commands.get("start").is_some());
        assert!(config.commands.get("stringCmd").is_some());
    }
}
