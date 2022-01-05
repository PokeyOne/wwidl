use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigLoadError {
    UnsupportedSystem
}

#[derive(Debug, Clone)]
pub struct Config {

}

impl Config {
    pub fn load() -> Result<Config, ConfigLoadError> {
        let os = match env::var("OS") {
            Ok(os) => os,
            Err(_) => "unknown".to_string(),
        };

        println!("OS: {}", os);
        todo!()
    }
}
