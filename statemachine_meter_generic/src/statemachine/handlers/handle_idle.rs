use tokio::time::sleep;
use std::time::Duration;
use crate::statemachine::{StateMachine, State};

pub async fn handle_idle(state_machine: &mut StateMachine) {
    println!("State: IDLE");
    sleep(Duration::from_secs(5)).await;

    if state_machine.first_idle {
        state_machine.state = State::Ping;
        state_machine.first_idle = false;
    } else {
        state_machine.state = State::Read;
    }
}
