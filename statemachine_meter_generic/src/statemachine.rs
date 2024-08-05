// statemachine_meter_generic/src/statemachine.rs

use std::sync::Arc;
use tokio::sync::Mutex;
mod handlers;
use handlers::{handle_idle, handle_ping, handle_read, handle_write, handle_verify, handle_modbus};
use config_meter_generic::config::Config;
use tokio_modbus::client::Context as ModbusContext;


#[derive(Debug)]
pub enum State {
    Idle,
    Ping,
    Modbus,
    Read,
    Write,
    Verify,
}

pub struct StateMachine {
    pub state: State,
    pub meter_data: Option<String>,
    pub write_data: Option<String>,
    pub modbus_context: Option<Arc<Mutex<ModbusContext>>>,
    first_idle: bool,
    config: Arc<Mutex<Config>>,
}

impl StateMachine {
    pub fn new(config: Arc<Mutex<Config>>) -> Arc<Mutex<Self>>  {
        Arc::new(Mutex::new(StateMachine {
            state: State::Idle,
            meter_data: None,
            write_data: None,
            first_idle: true,
            modbus_context: None,
            config,
        }))
    }

    pub async fn run(&mut self) {
        loop {
            match &self.state {
                State::Idle => handle_idle(self).await,
                State::Ping => handle_ping(self).await,
                State::Modbus => handle_modbus(self).await,
                State::Read => handle_read(self).await,
                State::Write => handle_write(self).await,
                State::Verify => handle_verify(self).await,
            }
        }
    }
}
