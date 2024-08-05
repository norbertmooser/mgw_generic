use tokio::time::sleep;
use std::time::Duration;
use crate::statemachine::StateMachine;

pub async fn handle_idle(_state_machine: &mut StateMachine) {
    println!("State: IDLE");
    sleep(Duration::from_secs(5)).await;

}
