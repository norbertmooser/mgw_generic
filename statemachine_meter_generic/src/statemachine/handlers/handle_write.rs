// statemachine_meter_generic/src/statemachine/handlers/handle_write.rs

use tokio_modbus::prelude::*;
use crate::statemachine::{StateMachine, State};
use std::error::Error;
use config_meter_generic::config::ConfigWriteRegister;
use tokio_modbus::client::Context;

pub async fn handle_write(state_machine: &mut StateMachine) {
    println!("State: WRITE");

    if let Some(_socket) = state_machine.modbus_context.as_ref() {
        println!("Using established socket connection for write operation.");
        match perform_write_operations(state_machine).await {
            Ok(_) => {
                println!("Write operation completed successfully.");
                state_machine.state = State::Verify;
            },
            Err(e) => {
                eprintln!("Write operation failed: {}", e);
                state_machine.state = State::Idle; // Transition to Idle on error
            }
        }
    } else {
        println!("No active socket connection found, unable to perform write operation.");
        state_machine.state = State::Idle; // Transition to Idle state
    }
}


// modbus_write function capable of handling various data types
async fn modbus_write(modbus_context: &mut Context, register: &ConfigWriteRegister) -> Result<(), Box<dyn Error>> {
    let address = register.address;
    let value = register.value; // Assuming value is already in the correct format for transmission

    // Convert float value to bytes assuming IEEE 754 floating-point format (common in Modbus TCP)
    let value_as_u16 = f32::to_le_bytes(value).chunks(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect::<Vec<u16>>();

    // Handle the write operation, correctly mapping errors
    match modbus_context.write_multiple_registers(address, &value_as_u16).await {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e) as Box<dyn Error + Send + Sync>)
    }
}



/// Adjusted perform_write_operations to use modbus_write function
async fn perform_write_operations(state_machine: &mut StateMachine) -> Result<(), Box<dyn Error>> {
    println!("Attempting to lock the configuration for writing.");
    let locked_config = state_machine.config.lock().await;
    println!("Configuration locked successfully.");

    let write_registers = locked_config.get_write_registers();
    println!("Write registers retrieved: {:?}", write_registers);

    if write_registers.is_empty() {
        println!("Error: No registers configured for writing.");
        return Err("No registers configured".into());
    }

    if let Some(ref mut modbus_context) = state_machine.modbus_context {
        let mut context = modbus_context.lock().await;
        println!("Modbus context is available, proceeding with the write operation.");
        for reg in &write_registers {
            println!("Writing value {} to address {}", reg.value, reg.address);
            if let Err(e) = modbus_write(&mut context, reg).await {
                println!("Failed to write register: {}", e);
                return Err(e);
            }
        }
    } else {
        println!("Modbus context not available. Cannot perform write operation.");
        return Err("Modbus context not available".into());
    }

    println!("Write operation completed successfully.");
    Ok(())
}
