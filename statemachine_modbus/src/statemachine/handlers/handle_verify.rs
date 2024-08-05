use tokio::time::{sleep, Duration, timeout};
use crate::statemachine::{StateMachine, State};
use tokio_modbus::prelude::Reader;
use log::{info, warn, error};

pub async fn handle_verify(state_machine: &mut StateMachine) {
    info!("Entering VERIFY state");

    if let Some(modbus_context) = &state_machine.modbus_context {
        info!("Modbus context found, attempting to lock");

        // Setting a timeout duration for locking the context
        let lock_timeout = Duration::from_secs(5);

        match timeout(lock_timeout, modbus_context.lock()).await {
            Ok(mut context) => {
                info!("Modbus context locked, attempting to read holding registers");

                match context.read_holding_registers(0, 1).await {
                    Ok(_) => {
                        // Connection is active
                        info!("Modbus connection is active");
                        drop(context); // Release the lock
                        info!("Context lock released, sleeping for 10 seconds");
                        sleep(Duration::from_secs(10)).await;
                        state_machine.state = State::Verify; // Remain in Verify state
                        info!("Rechecking the connection, remaining in VERIFY state");
                    }
                    Err(_) => {
                        // Connection is not active
                        warn!("Failed to read holding registers, modbus connection is not active");
                        state_machine.state = State::Idle;
                        info!("Switching state to IDLE");
                    }
                }
            }
            Err(_) => {
                // Locking the context timed out
                error!("Failed to lock modbus context within timeout duration, switching state to IDLE");
                state_machine.state = State::Idle;
            }
        }
    } else {
        warn!("Modbus context not found, switching state to IDLE");
        state_machine.state = State::Idle;
    }

    info!("Exiting VERIFY state");
}
