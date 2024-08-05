#!/bin/bash

# Define the tarball name
TARBALL_NAME="mgw_generic.tar.gz"

# Check if the tarball already exists and delete it if it does
if [ -f "$TARBALL" ]; then
    echo "Tarball $TARBALL already exists. Deleting it."
    rm "$TARBALL"
fi


# Create the tarball, including necessary files and directories
tar -czvf $TARBALL_NAME \
    Cargo.toml \
    Cargo.lock \
    commit_and_push.sh \
    create_tarball.sh \
    mgw_config.yaml \
    src/main.rs \
    config_meter_generic/Cargo.toml \
    config_meter_generic/src/config.rs \
    config_meter_generic/src/lib.rs \
    modbus_meter_generic/Cargo.toml \
    modbus_meter_generic/Cargo.lock \
    modbus_meter_generic/src/lib.rs \
    modbus_meter_generic/src/meter.rs \
    pinger/Cargo.toml \
    pinger/src/lib.rs \
    pinger/src/pinger.rs \
    statemachine_meter_generic/Cargo.toml \
    statemachine_meter_generic/src/statemachine.rs \
    statemachine_meter_generic/src/lib.rs \
    statemachine_meter_generic/src/statemachine/handlers/handle_idle.rs \
    statemachine_meter_generic/src/statemachine/handlers/handle_ping.rs \
    statemachine_meter_generic/src/statemachine/handlers/handle_read.rs \
    statemachine_meter_generic/src/statemachine/handlers/handle_modbus.rs \
    statemachine_meter_generic/src/statemachine/handlers/handle_verify.rs \
    statemachine_meter_generic/src/statemachine/handlers/handle_write.rs \
    statemachine_meter_generic/src/statemachine/handlers/mod.rs \
    test_ping/Cargo.toml \
    test_ping/src/main.rs

# Print a message indicating completion
echo "Tarball $TARBALL_NAME created successfully."


rm ~/mount/df/mgw_generic.tar.gz
cp mgw_generic.tar.gz ~/mount/df/

