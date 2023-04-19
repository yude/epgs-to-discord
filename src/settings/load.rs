use config::{Config, ConfigError, File};
use serde_derive::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Credentials {
    pub webhook_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Misc {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub credentials: Credentials,
    pub misc: Misc,
}

impl Settings {
    pub fn new(parent_path: &String) -> Result<Self, ConfigError> {
        let mut home_dir: String = "".to_string();
        match home::home_dir() {
            Some(path) => home_dir = path.display().to_string(),
            None => println!("Impossible to get your home dir!"),
        }

        let config_path = format!("{}/.config/epgs-to-discord/config.toml", &home_dir);

        if !Path::new(&config_path).exists() {
            println!("Failed to locate config.toml: {}", parent_path);
            std::process::exit(1);
        }

        let s = Config::builder()
            .add_source(File::with_name(&config_path))
            .build()?;

        s.try_deserialize()
    }
}
