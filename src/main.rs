use rumqttc::{Client, MqttOptions, QoS};
use std::{thread, time::Duration};

fn main() {
    // MQTT setup
    let mut mqttoptions = MqttOptions::new(
        "rust_simulator",
        "thingsboard.cloud",
        1883,
    );
    mqttoptions.set_credentials("nzxtAUUMLaGSNQ2672LR", "");

    // Create client and connection
    let (client, mut connection) = Client::new(mqttoptions, 10);

    // Thread to handle incoming notifications (optional)
    std::thread::spawn(move || {
        for notification in connection.iter() {
            println!("Notification: {:?}", notification);
        }
    });

    let mut value = 10;
    let mut up = true;

    // NEW: water level variable
    let mut water_level = 50;

    loop {

        // UPDATED: payload now sends two sensors
        let payload = format!(
            "{{\"temperature\": {}, \"water_level\": {}}}",
            value,
            water_level
        );

        println!("Sending {}", payload);

        // Publish to ThingsBoard
        client
            .publish("v1/devices/me/telemetry", QoS::AtLeastOnce, false, payload)
            .unwrap();

        // Temperature simulation
        if up {
            value += 1;
            if value >= 30 {
                up = false;
            }
        } else {
            value -= 1;
            if value <= 10 {
                up = true;
            }
        }

        // NEW: water level simulation
        water_level += 2;

        if water_level > 100 {
            water_level = 50;
        }

        // Wait 10 seconds for fast testing
        thread::sleep(Duration::from_secs(10));
    }
}