use std::env;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    repos: Vec<RepoData>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoData {
    path: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            repos: vec![],
        }
    }

    pub fn load() -> Self {
        let system = env::consts::OS;
        println!("System: {}", system);
        if system == "windows" {
            // TODO: !MC - Figure out where to store configs on windows
            println!("Windows Operating System is not fully supported yet. Loading default config.");
            return Self::default()
        }

        let home_dir = match env::var("HOME") {
            Ok(path) => {
                let path = PathBuf::from(path);

                if path.exists() {
                    path
                } else {
                    println!("Home directory does not exist. Loading default config.");
                    return Self::default()
                }
            }
            Err(_) => {
                println!("Could not find home directory. Ensure HOME env variable is set. Loading default config.");
                return Self::default()
            }
        };

        let config_path = home_dir.join(".wwidl").join("config.toml");
        println!("Loading config from: {:?}", config_path);

        if config_path.exists() {
            println!("Config file exists. Loading config.");
            let mut config: Config = toml::from_str(std::fs::read_to_string(config_path).unwrap().as_str()).unwrap();
            return config;
        } else {
            println!("Config file does not exist. Creating default config.");
            let mut default_config = Self::default();

            // TODO: Errors should be handled by just printing a message and exiting
            std::fs::create_dir_all(home_dir.join(".wwidl")).unwrap();
            std::fs::write(&config_path, toml::to_string(&default_config).unwrap()).unwrap();

            default_config
        }
    }
}
