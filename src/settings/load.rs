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
        let config_path = &(parent_path.to_owned() + "/config.toml");

        if !Path::new(config_path).exists() {
            println!("Failed to locate config.toml: {}", parent_path);
            std::process::exit(1);
        }

        let s = Config::builder()
            .add_source(File::with_name(config_path))
            .build()?;

        s.try_deserialize()
    }
}
