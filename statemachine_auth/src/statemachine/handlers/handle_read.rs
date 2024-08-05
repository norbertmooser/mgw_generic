// mgw_generic/statemachine_meter_generic/src/statemachine/handlers/handle_read.rs

use tokio_modbus::prelude::*;
use crate::statemachine::{StateMachine, State};
use std::error::Error;
use config_meter_generic::config::ConfigRegister;

/// Handles the READ operation within the state machine.
pub async fn handle_read(state_machine: &mut StateMachine) {
    println!("State: READ");

    if let Some(_socket) = state_machine.modbus_context.as_ref() {
        println!("Using established socket connection for read operation.");
        match perform_read_operations(state_machine).await {
            Ok(_) => {
                println!("Read operation completed successfully.");
                let locked_config = state_machine.config.lock().await;
                let config = &*locked_config; // Dereference to access the content
                if config.has_write_registers() {
                    println!("Write registers are available, transitioning to WRITE state.");
                    state_machine.state = State::Write;
                } else {
                    println!("No write registers available, transitioning to IDLE state.");
                    state_machine.state = State::Idle;
                }
            },
            Err(e) => {
                eprintln!("Read operation failed: {}", e);
                state_machine.state = State::Idle; // Transition to Idle on error
            }
        }
    } else {
        println!("No active socket connection found, unable to perform read operation.");
        state_machine.state = State::Idle; // Transition to Idle state
    }
}

/// Decodes and displays register values based on read operations.
/// 
/// # Arguments
/// * `read_registers` - A slice of `ConfigRegister` holding configuration for registers.
/// * `values_vec` - A slice of `u16` holding the values read from Modbus registers.
/// * `start_address` - The starting address of the read operation.
/// 
/// # Errors
/// Returns an error if any register is out of bounds.
fn decode_and_display_values(
    read_registers: &[ConfigRegister],
    values_vec: &[u16],
    start_address: u16,
) -> Result<(), Box<dyn Error>> {
    for register in read_registers {
        let offset = (register.address - start_address) as usize;
        if offset + 1 >= values_vec.len() {
            return Err(format!("Index out of bounds for register: {}", register.name).into());
        }
        let high = values_vec[offset] as u32;
        let low = values_vec[offset + 1] as u32;
        let value = f32::from_bits((high << 16) | low);
        println!("Name: {}, Address: {}, Value: {}", register.name, register.address, value);
    }
    Ok(())
}

/// Performs read operations for the state machine, managing configurations and Modbus interactions.
async fn perform_read_operations(state_machine: &mut StateMachine) -> Result<(), Box<dyn Error>> {
    println!("Attempting to lock the configuration for reading.");
    let locked_config: tokio::sync::MutexGuard<config_meter_generic::config::Config> = state_machine.config.lock().await;
    println!("Configuration locked successfully.");

    let config: &config_meter_generic::config::Config = &*locked_config;
    println!("Configuration accessed.");

    let read_registers: Vec<ConfigRegister> = config.get_read_registers();
    println!("Read registers retrieved: {:?}", read_registers);

    if read_registers.is_empty() {
        println!("Error: No registers configured for reading.");
        return Err("No registers configured".into());
    }

    let start_address = read_registers.iter().map(|r| r.address).min().unwrap();
    let end_address = read_registers.iter().map(|r| r.address).max().unwrap();
    let quantity = (end_address - start_address + 2) as u16;

    println!("Determined range of registers to read:");
    println!("  - Start address: {}", start_address);
    println!("  - End address: {}", end_address);
    println!("  - Quantity to read: {}", quantity);

    if let Some(ref modbus_context) = state_machine.modbus_context {
        let mut context = modbus_context.lock().await;
        println!("Modbus context is available, proceeding with the read operation.");
        match context.read_holding_registers(start_address, quantity).await {
            Ok(values) => {
                println!("Successfully read values from Modbus device:");
                decode_and_display_values(&read_registers, &values?, start_address)?;
            },
            Err(e) => {
                println!("Failed to read registers: {}", e);
                return Err(e.into());
            }
        }
    } else {
        println!("Modbus context not available. Cannot perform read operation.");
        return Err("Modbus context not available".into());
    }

    println!("Read operation completed successfully.");
    Ok(())
}

