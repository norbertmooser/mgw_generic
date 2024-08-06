use std::sync::Arc;
use tokio::sync::Mutex;
use config_meter_generic::config::Config;
use anyhow::Result;
use log::{error, info, LevelFilter};
use std::str::FromStr;

pub async fn update_log_levels(config_path: &str, shared_config: Arc<Mutex<Config>>) -> Result<()> {
    loop {
        // Load the configuration and handle errors with context
        match Config::from_file(config_path) {
            Ok(new_config) => {
                {
                    let mut config = shared_config.lock().await;
                    *config = new_config;
                }

                {
                    let config = shared_config.lock().await;
                    log::set_max_level(LevelFilter::from_str(&config.debug.mgw_generic).unwrap_or(LevelFilter::Info));
                    log::set_max_level(LevelFilter::from_str(&config.debug.statemachine_modbus).unwrap_or(LevelFilter::Info));
                    log::set_max_level(LevelFilter::from_str(&config.debug.statemachine_read).unwrap_or(LevelFilter::Info));
                    info!("Log levels updated from configuration");
                }
            }
            Err(e) => {
                error!("Failed to update configuration: {:?}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}
