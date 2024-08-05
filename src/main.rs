// src/main.rs

use std::sync::Arc;
use tokio::sync::Mutex;
use config_meter_generic::config::Config;
use statemachine_meter_generic::statemachine::StateMachine as StateMachineMeterGeneric;
// use statemachine_auth::statemachine::StateMachine as StateMachineAuth;
use anyhow::Result; // Brings in the anyhow Result type
use anyhow::Context;

#[tokio::main]
async fn main() -> Result<()> {
    let config_path = "mgw_config.yaml";

    // Load the configuration and handle errors with context
    let config = Config::from_file(config_path)
        .with_context(|| format!("Failed to load configuration from {}", config_path))?;

    let shared_config: Arc<Mutex<Config>> = Arc::new(Mutex::new(config));
    let state_machine_meter_generic: Arc<Mutex<StateMachineMeterGeneric>> = StateMachineMeterGeneric::new(Arc::clone(&shared_config));

    // Obtain the state machine in a mutable form to call run
    let mut sm_meter_generic = state_machine_meter_generic.lock().await;
    sm_meter_generic.run().await; // Start the state machine's loop
    


    Ok(())
}
