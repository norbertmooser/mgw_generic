use tokio::time::sleep;
use std::time::Duration;
use crate::statemachine::{StateMachine, State};
use log::info;

pub async fn handle_idle(state_machine: &mut StateMachine) {
    info!("State: IDLE");
    sleep(Duration::from_secs(5)).await;
    state_machine.state = State::Ping;
    info!("Transitioning to State: PING");
}
