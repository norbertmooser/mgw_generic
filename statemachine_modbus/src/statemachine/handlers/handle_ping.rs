use crate::statemachine::{StateMachine, State};
use std::error::Error;

pub async fn handle_ping(state_machine: &mut StateMachine) {
    println!("State: PING");

    // Access the shared configuration safely
    let config = state_machine.config.lock().await;
    let ip_address = &config.meter_data.ip;  // Get the IP address from the configuration

    let ping_result = ping_meter(ip_address).await;
    match ping_result {
        Ok(_) => {
            state_machine.state = State::Connect;
        }
        Err(_) => {
            state_machine.state = State::Idle;
        }
    }
}



async fn ping_meter(ip_address: &str) -> Result<(), Box<dyn Error>> {
    use tokio::process::Command;
    // Start of the ping operation
    println!("Attempting to ping IP address: {}", ip_address);

    // Execute the ping command using tokio::process::Command
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip_address)  // Dynamically use the IP address passed to the function
        .output()
        .await;

    match output {
        Ok(output) => {
            // Command executed successfully, check if the ping was successful
            if output.status.success() {
                // Ping command succeeded
                println!("Ping to {} was successful.", ip_address);
                Ok(())
            } else {
                // Ping command failed
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Ping to {} failed, error: {}", ip_address, stderr);
                Err("Ping failed".into())
            }
        },
        Err(e) => {
            // Command failed to execute
            eprintln!("Failed to execute ping command to {}: {}", ip_address, e);
            Err(e.into())
        }
    }
}


