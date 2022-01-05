use std::env;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigLoadError {
    UnsupportedSystem
}

#[derive(Debug, Clone)]
pub struct Config {

}

impl Config {
    pub fn default() -> Self {
        Config { }
    }

    pub fn load() -> Result<Self, ConfigLoadError> {
        let system = env::consts::OS;
        println!("System: {}", system);
        if system == "windows" {
            // TODO: !MC - Figure out where to store configs on windows
            println!("Windows Operating System is not fully supported yet. Loading default config.");
            return Ok(Self::default())
        }

        let home_dir = match env::var("HOME") {
            Ok(path) => path,
            Err(_) => {
                println!("Could not find home directory. Ensure HOME env variable is set. Loading default config.");
                return Ok(Self::default());
            }
        };

        let config_dir = home_dir + "/.wwidl/config.toml";
        println!("Loading config from: {}", config_dir);

        todo!()
    }
}
