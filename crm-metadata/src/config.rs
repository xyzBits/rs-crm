use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::env::current_dir;
use std::fs::File;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    pub pk: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let dir = current_dir()?;
        info!("load metadata app config current dir is {:?}", dir);
        let ret = match (
            File::open("crm-metadata/metadata.yml"), // main 函数运行时，根目录在 rs-crm
            File::open("metadata.yml"), // 单元测试运行时，根目录在 user-stat，直接在 idea 中运行，不是命令行中运行
            File::open("/etc/config/metadata.yml"),
            env::var("USER_STAT_CONFIG"),
        ) {
            (Ok(reader), _, _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, _, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };

        Ok(ret?)
    }
}

#[test]
fn test_load_metadata_config() -> Result<()> {
    // 单元测试运行时，根目录在 user-stat
    // main 函数运行时，根目录在 rs-crm
    let app_config = AppConfig::load()?; // 在 idea 中运行测试，而不是在命令行
    println!("{:?}", app_config);

    Ok(())
}
