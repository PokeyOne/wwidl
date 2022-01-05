#[cfg(test)]
mod tests;

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

    pub fn determine_config_path() -> Option<PathBuf> {
        let home_dir = match env::var("HOME") {
            Ok(path) => {
                let path = PathBuf::from(path);

                if path.exists() {
                    path
                } else {
                    println!("Home directory does not exist. Loading default config.");
                    return None
                }
            }
            Err(_) => {
                println!("Could not find home directory. Ensure HOME env variable is set. Loading default config.");
                return None
            }
        };

        let config_path = home_dir.join(".wwidl").join("config.toml");
        Some(config_path)
    }

    pub fn load() -> Self {
        let config_path = match Self::determine_config_path() {
            Some(path) => path,
            None => {
                println!("Could not determine config path. Loading default config.");
                return Self::default();
            }
        };

        Self::load_from_path(config_path)
    }

    pub fn load_from_path(config_path: PathBuf) -> Self {
        if config_path.exists() {
            println!("Config file exists. Loading config.");
            let config: Config = toml::from_str(std::fs::read_to_string(config_path).unwrap().as_str()).unwrap();
            return config;
        } else {
            println!("Config file does not exist. Creating default config.");
            let default_config = Self::default();

            // TODO: Errors should be handled by just printing a message and exiting
            std::fs::create_dir_all(config_path.parent().unwrap()).unwrap();
            std::fs::write(&config_path, toml::to_string(&default_config).unwrap()).unwrap();

            default_config
        }
    }

    pub fn save(&self) {
        let config_path = match Self::determine_config_path() {
            Some(path) => path,
            None => {
                println!("Could not determine config path. Cannot save config. Config that would have been saved will be printed below.");
                println!("{}", toml::to_string(&self).unwrap());
                return;
            }
        };

        // TODO: handle write errors
        std::fs::write(config_path, toml::to_string(&self).unwrap()).unwrap();
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

impl RepoData {
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Returns a reference to the vector of messages relating to this repo
    pub fn messages(&self) -> &Vec<MessageData> {
        &self.messages
    }

    pub fn last_message(&self) -> Option<&MessageData> {
        self.messages().last()
    }
}

impl MessageData {
    pub fn date(&self) -> &Datetime {
        &self.date
    }

    pub fn message(&self) -> &str {
        &self.message
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