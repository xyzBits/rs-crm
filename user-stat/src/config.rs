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
    pub db_url: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let dir = current_dir()?;
        info!("load app config current dir is {:?}", dir);
        let ret = match (
            File::open("user-stat/user_stat.yml"), // 程序运行时，其目录为项目的根目录
            File::open("/etc/config/user_stat.yml"),
            env::var("USER_STAT_CONFIG"),
        ) {
            (Ok(reader), _, _) => serde_yaml::from_reader(reader),
            (_, Ok(reader), _) => serde_yaml::from_reader(reader),
            (_, _, Ok(path)) => serde_yaml::from_reader(File::open(path)?),
            _ => bail!("Config file not found"),
        };

        Ok(ret?)
    }
}

#[test]
fn test_load_config() -> Result<()> {
    // 单元测试运行时，根目录在 user-stat
    // main 函数运行时，根目录在 rs-crm
    // let app_config = AppConfig::load()?;
    // println!("{:?}", app_config);

    Ok(())
}
