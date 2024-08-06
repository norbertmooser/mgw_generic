use tokio::time::sleep;
use std::time::Duration;
use crate::statemachine::{StateMachine, State};
use log::info;

pub async fn handle_idle(state_machine: &mut StateMachine) {
    info!("Entering IDLE state");

    // Simulating idle period
    sleep(Duration::from_secs(5)).await;

    if state_machine.first_idle {
        info!("First time in IDLE state, transitioning to READ state");
        state_machine.state = State::Read;
        state_machine.first_idle = false;
    } else {
        info!("Returning to READ state");
        state_machine.state = State::Read;
    }
}
