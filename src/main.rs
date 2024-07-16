use modbus_meter_generic::meter::MeterGeneric;
use config_meter_generic::config::{Config, ConfigRegister, ConfigWriteRegister};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "mgw_config.yaml";
    let ip = "10.15.1.2";
    let port = 502;

    let config = Config::from_file(config_path)?;
    let read_registers: Vec<ConfigRegister> = config.read_registers;
    let write_registers: Vec<ConfigWriteRegister> = config.write_registers;

    
    let mut meter = MeterGeneric::new(read_registers, write_registers);
    meter.connect(ip, port).await?;
    meter.write().await?;
    meter.read().await?;

    Ok(())
}
