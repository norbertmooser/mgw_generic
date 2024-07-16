use modbus_meter_generic::meter::MeterGeneric;
use config_meter_generic::config::{Config, ConfigRegister, ConfigWriteRegister, validate_ip_and_port};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "mgw_config.yaml";

    let config = Config::from_file(config_path)?;
    let (ip, port) = config.get_ip_and_port()?;
    validate_ip_and_port(&ip, port)?;

    let read_registers: Vec<ConfigRegister> = config.get_read_registers();
    let write_registers: Vec<ConfigWriteRegister> = config.get_write_registers();

    let mut meter = MeterGeneric::new(read_registers, write_registers);
    meter.connect(&ip, port).await?;
    meter.write().await?;
    meter.read().await?;

    Ok(())
}
