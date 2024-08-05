use std::sync::Arc;
use tokio::sync::Mutex;
use config_meter_generic::config::Config;
use statemachine_modbus::statemachine::StateMachine as StateMachineModbus;
use anyhow::Result; // Brings in the anyhow Result type
use anyhow::Context;
use log::{info, error};
use env_logger::Builder;
use chrono::Local;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the logger with a custom format and default level
    Builder::new()
        .format(|buf, record| {
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
        })
        .filter(None, log::LevelFilter::Info)  // Default log level to Info
        .init();

    info!("Starting the application...");

    let config_path = "mgw_config.yaml";
    info!("Loading configuration from {}", config_path);

    // Load the configuration and handle errors with context
    let config = match Config::from_file(config_path)
        .with_context(|| format!("Failed to load configuration from {}", config_path)) {
            Ok(cfg) => {
                info!("Configuration loaded successfully");
                cfg
            }
            Err(e) => {
                error!("Error loading configuration: {:?}", e);
                return Err(e);
            }
        };

    let shared_config: Arc<Mutex<Config>> = Arc::new(Mutex::new(config));
    info!("Shared configuration created");

    let state_machine_modbus: Arc<Mutex<StateMachineModbus>> = StateMachineModbus::new(Arc::clone(&shared_config));
    info!("State machines created");

    // Start the state machine concurrently using tokio::spawn
    let sm1 = tokio::spawn(async move {
        info!("Starting state machine");
        let mut sm = state_machine_modbus.lock().await;
        sm.run().await;
    });

    // Await the state machine to complete
    let _ = tokio::join!(sm1);

    info!("Application finished");

    Ok(())
}
