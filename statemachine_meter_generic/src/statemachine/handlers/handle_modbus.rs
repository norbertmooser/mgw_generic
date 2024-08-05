// statemachine_meter_generic/src/statemachine/handlers/handle_modbus.rs
use std::error::Error;
use std::fmt;
use std::net::SocketAddr;
use tokio::time::{self, Duration};
use tokio_modbus::prelude::*;
use tokio_modbus::client::{tcp, Context};
use std::sync::Arc;
use tokio::sync::Mutex;


use crate::statemachine::{StateMachine, State};
use config_meter_generic::config::validate_ip_and_port;

/// Custom error type for Modbus operations.
#[derive(Debug)]
#[allow(dead_code)]
enum SocketError {
    Timeout,
    ConnectionFailed(String),
    Other(Box<dyn Error>), // Consider whether this variant is necessary or should be removed
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SocketError::Timeout => write!(f, "Connection attempt timed out"),
            SocketError::ConnectionFailed(msg) => write!(f, "Failed to connect: {}", msg),
            SocketError::Other(e) => write!(f, "An error occurred: {}", e),
        }
    }
}

impl Error for SocketError {}

/// Checks if the Modbus context is still active by attempting to read a known register.
async fn is_context_alive(context: &mut Context) -> bool {
    const STATUS_REGISTER_ADDRESS: u16 = 0x0001;
    context.read_holding_registers(STATUS_REGISTER_ADDRESS, 1).await.is_ok()
}

/// Attempts to establish a new Modbus context to the specified IP and port, with a timeout of 5 seconds.
async fn setup_modbus_context(ip: &str, port: u16) -> Result<Arc<Mutex<Context>>, Box<dyn std::error::Error>> {
// async fn setup_modbus_context(ip: &str, port: u16) -> Result<Context, SocketError> {
    let address = format!("{}:{}", ip, port).parse::<SocketAddr>().map_err(|e| SocketError::ConnectionFailed(e.to_string()))?;
    println!("Attempting to connect to the Modbus device at {}", address);

    match time::timeout(Duration::from_secs(5), tcp::connect(address)).await {
        Ok(Ok(context)) => {
            println!("Modbus TCP connection established.");
            Ok(Arc::new(Mutex::new(context)))

        },
        Ok(Err(e)) => {
            println!("Failed to connect to the Modbus device: {}", e);
            Err(Box::new(SocketError::ConnectionFailed(e.to_string())))
        },
        Err(_) => {
            println!("Connection attempt to Modbus device timed out after 5 seconds.");
            Err(Box::new(SocketError::Timeout))
        }
    }
}

/// Handles the Modbus connection logic based on the current state of the state machine.

pub async fn handle_modbus(state_machine: &mut StateMachine) {
    println!("State: Modbus Connection Handling");

    if let Some(ref modbus_context) = state_machine.modbus_context {
        let mut context = modbus_context.lock().await;
        if is_context_alive(&mut context).await {
            println!("Modbus context is active, transitioning to READ state.");
            state_machine.state = State::Read;
            return;
        } else {
            println!("Modbus context is not active, unable to perform read operation.");
            state_machine.state = State::Idle;
            return;
        }
    }

    println!("No Modbus context found, attempting to establish connection.");
    let locked_config = state_machine.config.lock().await;
    let config = &*locked_config;

    if let Err(e) = validate_ip_and_port(&config.meter_data.ip, config.meter_data.port) {
        eprintln!("Invalid IP or port: {}", e);
        state_machine.state = State::Idle;
        return;
    }

    match setup_modbus_context(&config.meter_data.ip, config.meter_data.port).await {
        Ok(context) => {
            println!("Modbus connection established.");
            state_machine.modbus_context = Some(context);
            state_machine.state = State::Read;
        },
        Err(e) => {
            eprintln!("Failed to establish Modbus connection: {}", e);
            state_machine.first_idle = true;
            state_machine.state = State::Idle;
        }
    }
}
