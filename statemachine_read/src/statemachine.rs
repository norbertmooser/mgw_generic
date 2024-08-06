use std::sync::Arc;
use tokio::sync::Mutex;
mod handlers;
use handlers::{handle_idle, handle_read};
use config_meter_generic::config::Config;
use statemachine_modbus::statemachine::StateMachine as StateMachineModbus;
use log::info;

#[derive(Debug)]
pub enum State {
    Idle,
    Read,
}

pub struct StateMachine {
    pub state: State,
    pub meter_data: Option<String>,
    first_idle: bool,
    config: Arc<Mutex<Config>>,
    modbus_statemachine: Arc<Mutex<StateMachineModbus>>,
}

impl StateMachine {
    pub fn new(config: Arc<Mutex<Config>>, modbus_statemachine: Arc<Mutex<StateMachineModbus>>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(StateMachine {
            state: State::Idle,
            meter_data: None,
            first_idle: true,
            config,
            modbus_statemachine,
        }))
    }

    pub async fn run(&mut self) {
        loop {
            info!("Current state: {:?}", self.state);
            match &self.state {
                State::Idle => {
                    info!("Entering Idle state");
                    handle_idle(self).await;
                }
                State::Read => {
                    info!("Entering Read state");
                    handle_read(self).await;
                }
            }
            info!("Transitioning to next state: {:?}", self.state);
        }
    }
}
