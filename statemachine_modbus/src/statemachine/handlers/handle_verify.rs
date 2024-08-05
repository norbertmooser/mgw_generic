use crate::statemachine::{StateMachine, State};
use std::error::Error;

pub async fn handle_verify(state_machine: &mut StateMachine) {
    println!("State: VERIFY");
    let _verify_result = verify_written_data().await;
    state_machine.state = State::Idle;
}

async fn verify_written_data() -> Result<(), Box<dyn Error>> {
    use tokio::time::sleep;
    use std::time::Duration;

    // Simulate verifying written data
    sleep(Duration::from_secs(2)).await;
    println!("Verifying written data...");
    Ok(())
}
