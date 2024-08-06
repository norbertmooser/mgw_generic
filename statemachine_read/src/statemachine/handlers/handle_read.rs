use crate::statemachine::{StateMachine, State};
use std::error::Error;
use config_meter_generic::config::ConfigRegister;
use tokio_modbus::client::Context as ModbusContext;
use tokio_modbus::prelude::Reader;
use log::{info, error};

/// Handles the READ operation within the state machine.
pub async fn handle_read(state_machine: &mut StateMachine) {
    info!("State: READ");

    // Limit the scope of the immutable borrow of state_machine
    let modbus_context_option = {
        let lock = state_machine.modbus_statemachine.lock().await;
        lock.access_modbus_context().await
    };

    // Await the future and handle the Result
    match modbus_context_option {
        Ok(Some(modbus_context)) => {
            let mut context = modbus_context.lock().await;
            info!("Using established socket connection for read operation.");
            match perform_read_operations(state_machine, &mut context).await {
                Ok(_) => {
                    info!("Read operation completed successfully.");
                    state_machine.state = State::Idle;
                },
                Err(e) => {
                    error!("Read operation failed: {}", e);
                    state_machine.state = State::Idle; // Transition to Idle on error
                }
            }
        },
        Ok(None) => {
            info!("No active socket connection found, unable to perform read operation.");
            state_machine.state = State::Idle; // Transition to Idle state
        },
        Err(e) => {
            error!("Error accessing Modbus context: {}", e);
            state_machine.state = State::Idle; // Transition to Idle on error
        }
    }
}

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
        info!("Name: {}, Address: {}, Value: {}", register.name, register.address, value);
    }
    Ok(())
}

/// Performs read operations for the state machine, managing configurations and Modbus interactions.
async fn perform_read_operations(state_machine: &mut StateMachine, context: &mut ModbusContext) -> Result<(), Box<dyn Error>> {
    info!("Attempting to lock the configuration for reading.");
    let locked_config = state_machine.config.lock().await;
    info!("Configuration locked successfully.");

    let config = &*locked_config;
    info!("Configuration accessed.");

    let read_registers: Vec<ConfigRegister> = config.get_read_registers();
    info!("Read registers retrieved: {:?}", read_registers);

    if read_registers.is_empty() {
        error!("No registers configured for reading.");
        return Err("No registers configured".into());
    }

    let start_address = read_registers.iter().map(|r| r.address).min().unwrap();
    let end_address = read_registers.iter().map(|r| r.address).max().unwrap();
    let quantity = (end_address - start_address + 2) as u16;

    info!("Determined range of registers to read:");
    info!("  - Start address: {}", start_address);
    info!("  - End address: {}", end_address);
    info!("  - Quantity to read: {}", quantity);

    info!("Modbus context is available, proceeding with the read operation.");
    match context.read_holding_registers(start_address, quantity).await {
        Ok(values) => {
            info!("Successfully read values from Modbus device:");
            decode_and_display_values(&read_registers, &values?, start_address)?;
        },
        Err(e) => {
            error!("Failed to read registers: {}", e);
            return Err(e.into());
        }
    }

    info!("Read operation completed successfully.");
    Ok(())
}
