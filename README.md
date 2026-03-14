**Water Level & Temperature Monitoring with ESP32 (Rust)**
**Description**

This project uses an ESP32 microcontroller to read water level and temperature sensor data and sends it to ThingsBoard
 every 10 seconds.
It is implemented in Rust for efficient, safe, and reliable embedded control.

**Features**

Reads water level from sensor

Reads temperature from sensor

Sends data to ThingsBoard via HTTP/MQTT

Updates every 10 seconds

Written entirely in Rust

Easy to expand with additional sensors

**Hardware Requirements**

ESP32 DevKit board

Water level sensor (analog/digital as used in the project)

Temperature sensor (e.g., DS18B20 or similar)

Jumper wires and breadboard

**Software Requirements**

Rust (latest stable version)

cargo build tool

Git (for version control)

ThingsBoard account for data visualization  
**Installation**

Clone the repository:

git clone https://github.com/Mark-ai-dex/water_level_sim.git
cd water_level_sim

Build and run the project:

cargo run
