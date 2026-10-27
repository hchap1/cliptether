use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::DAEMON_NAME;
use crate::error::*;

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct Config {
    pub key_file: Option<PathBuf>,
}

pub async fn load_config() -> Res<Config> {
    let config_dir = match directories::ProjectDirs::from("", "", DAEMON_NAME) {
        Some(root) => root,
        None => return Err(Error::NoSuitableConfigLocation)
    }.config_dir().to_path_buf();

    let path = config_dir.join("config.toml");

    let config: Option<Config> = match tokio::fs::read_to_string(&path).await {
        Ok(string) => Some(toml::from_str(&string).map_err(|_| Error::InvalidConfig)?),
        Err(_) => None
    };

    match config {
        Some(config) => Ok(config),
        None => {
            tokio::fs::create_dir_all(config_dir)
                .await.map_err(|_| Error::NoSuitableConfigLocation)?;
            tokio::fs::write(&path, toml::to_string(&Config::default()).map_err(|_| Error::InvalidConfig)?)
                .await.map_err(|_| Error::NoSuitableConfigLocation)?;
            Ok(Config::default())
        }
    }
}

pub async fn write_config(config: Config) -> Res<()> {
    let config_dir = match directories::ProjectDirs::from("", "", DAEMON_NAME) {
        Some(root) => root,
        None => return Err(Error::NoSuitableConfigLocation)
    }.config_dir().to_path_buf();

    let path = config_dir.join("config.toml");

    tokio::fs::create_dir_all(config_dir)
        .await.map_err(|_| Error::NoSuitableConfigLocation)?;
    tokio::fs::write(&path, toml::to_string(&config).map_err(|_| Error::InvalidConfig)?)
        .await.map_err(|_| Error::NoSuitableConfigLocation)?;

    Ok(())
}
