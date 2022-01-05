use std::env;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml::value::Datetime;
use chrono::{DateTime as ChronoDateTime, Utc};
use toml::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    repos: Vec<RepoData>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoData {
    path: String,
    messages: Vec<MessageData>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MessageData {
    message: String,
    date: Datetime
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
            let config: Config = toml::from_str(std::fs::read_to_string(config_path).unwrap().as_str()).unwrap();
            return config;
        } else {
            println!("Config file does not exist. Creating default config.");
            let default_config = Self::default();

            // TODO: Errors should be handled by just printing a message and exiting
            std::fs::create_dir_all(home_dir.join(".wwidl")).unwrap();
            std::fs::write(&config_path, toml::to_string(&default_config).unwrap()).unwrap();

            default_config
        }
    }

    pub fn repo_data_mut(&mut self, repo_path: &str) -> Option<&mut RepoData> {
        self.repos.iter_mut().find(|repo| repo.path == repo_path)
    }

    pub fn repo_data(&self, repo_path: &str) -> Option<&RepoData> {
        self.repos.iter().find(|repo| repo.path == repo_path)
    }

    pub fn put_note(&mut self, repo_path: &str, note: String) {
        match self.repo_data_mut(repo_path) {
            Some(data) => {
                data.messages.push(MessageData {
                    message: note,
                    date: current_datetime()
                });
            }
            None => {
                self.repos.push(RepoData {
                    path: repo_path.to_string(),
                    messages: vec![MessageData {
                        message: note,
                        date: current_datetime()
                    }]
                });
            }
        }
    }
}

/// Returns the current time as a
/// [`Datetime`](https://docs.rs/toml/0.5.0/toml/value/struct.Datetime.html)
pub fn current_datetime() -> Datetime {
    // Probably isn't the most efficient thing to convert to a string, just
    // to parse it back to a Datetime. But it works and that can be an
    // optimization later.

    // Get the current time in UTC from chrono
    let now_time: ChronoDateTime<Utc> = Utc::now();
    // Convert the chrono datetime to a string in RFC3339 format for TOML and
    // assign it to a temporary property
    let now_string = format!("blah={}", now_time.to_rfc3339());
    // Parse the string as TOML into a Value type
    let value = now_string.parse::<Value>().unwrap();
    // Extract the Datetime from the Value type
    value["blah"].as_datetime().unwrap().clone()
}
