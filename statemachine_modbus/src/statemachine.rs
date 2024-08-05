// statemachine_meter_generic/src/statemachine.rs

use std::sync::Arc;
use tokio::sync::Mutex;
mod handlers;
use handlers::{handle_idle, handle_ping, handle_connect, handle_verify};
use config_meter_generic::config::Config;
use tokio_modbus::client::Context as ModbusContext;
use tokio::time::{timeout, Duration};
use anyhow::{Result, anyhow};


#[derive(Debug, PartialEq)]
pub enum State {
    Idle,
    Ping,
    Connect,
    Verify
}

pub struct StateMachine {
    pub state: State,
    pub meter_data: Option<String>,
    pub modbus_context: Option<Arc<Mutex<ModbusContext>>>,
    config: Arc<Mutex<Config>>,
}

impl StateMachine {
    pub fn new(config: Arc<Mutex<Config>>) -> Arc<Mutex<Self>>  {
        Arc::new(Mutex::new(StateMachine {
            state: State::Idle,
            meter_data: None,
            modbus_context: None,
            config,
        }))
    }

    pub async fn run(&mut self) {
        loop {
            match &self.state {
                State::Idle => handle_idle(self).await,
                State::Ping => handle_ping(self).await,
                State::Connect => handle_connect(self).await,
                State::Verify => handle_verify(self).await,
            }
        }
    }

    pub async fn access_modbus_context(&self) -> Result<()> {
        if self.state != State::Verify {
            return Err(anyhow!("State machine is not in VERIFY state"));
        }

        if let Some(modbus_context) = &self.modbus_context {
            println!("Attempting to lock Modbus context");

            let lock_timeout = Duration::from_secs(5);

            match timeout(lock_timeout, modbus_context.lock()).await {
                Ok(_context) => {
                    println!("Modbus context locked, validity check passed");
                    // The context is automatically freed here when it goes out of scope
                    Ok(())
                }
                Err(_) => {
                    println!("Failed to lock Modbus context within timeout duration");
                    Err(anyhow!("Timeout while attempting to lock Modbus context"))
                }
            }
        } else {
            println!("Modbus context not found");
            Err(anyhow!("Modbus context is not available"))
        }
    }

}
