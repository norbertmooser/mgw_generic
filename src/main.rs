mod update_log_levels;

use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use config_meter_generic::config::Config;
use anyhow::{Context, Result};
use log::{error, info, LevelFilter};
use env_logger::Builder;
use chrono::Local;
use std::io::Write;
use update_log_levels::update_log_levels;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path: &str = "mgw_config.yaml";
    info!("Loading configuration from {}", config_path);

    // Load the initial configuration and handle errors with context
    let config: Config = match Config::from_file(config_path)
        .with_context(|| format!("Failed to load configuration from {}", config_path))
    {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            cfg
        }
        Err(e) => {
            error!("Error loading configuration: {:?}", e);
            return Err(e);
        }
    };

    // Initialize the logger with a custom format and initial log levels from the config
    let mut builder = Builder::new();
    builder.format(|buf, record| {
        let module_path = record.module_path().unwrap_or("unknown");
        let module_name = module_path.split("::").next().unwrap_or(module_path);

        writeln!(
            buf,
            "{} [{}] - {} - {}",
            Local::now().format("%H:%M:%S"),
            record.level(),
            record.args(),
            module_name,
        )
    });

    builder.filter(Some("mgw_generic"), LevelFilter::from_str(&config.debug.mgw_generic).unwrap_or(LevelFilter::Info));
    builder.filter(Some("statemachine_modbus"), LevelFilter::from_str(&config.debug.statemachine_modbus).unwrap_or(LevelFilter::Info));
    builder.filter(Some("statemachine_read"), LevelFilter::from_str(&config.debug.statemachine_read).unwrap_or(LevelFilter::Info));

    builder.init();

    info!("Starting the application...");

    let shared_config: Arc<Mutex<Config>> = Arc::new(Mutex::new(config));
    info!("Shared configuration created");

    // Start the log level update task
    let config_path = config_path.to_string();
    let shared_config_clone = Arc::clone(&shared_config);
    tokio::spawn(async move {
        if let Err(e) = update_log_levels(&config_path, shared_config_clone).await {
            error!("Failed to update log levels: {:?}", e);
        }
    });

    let state_machine_modbus = statemachine_modbus::StateMachine::new(Arc::clone(&shared_config));
    let state_machine_read =
        statemachine_read::StateMachine::new(Arc::clone(&shared_config), Arc::clone(&state_machine_modbus));

    info!("State machines created");

    // Start the state machines concurrently using tokio::spawn
    let sm1: tokio::task::JoinHandle<()> = tokio::spawn({
        let state_machine_modbus = Arc::clone(&state_machine_modbus);
        async move {
            info!("Starting state machine modbus");
            let mut sm = state_machine_modbus.lock().await;
            sm.run().await;
        }
    });

    let sm2: tokio::task::JoinHandle<()> = tokio::spawn({
        let state_machine_read = Arc::clone(&state_machine_read);
        async move {
            info!("Starting state machine read");
            let mut sm = state_machine_read.lock().await;
            sm.run().await;
        }
    });

    // Await the state machines to complete
    let _ = tokio::join!(sm1, sm2);

    info!("Application finished");

    Ok(())
}
