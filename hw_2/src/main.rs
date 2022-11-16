mod smart_devices;

use smart_devices::smart_thermometer::{SmartThermometer};
use smart_devices::smart_socket::{SmartSocket, PowerMode};

fn main() {
    let socket_name = String::from("smart socket");
    let mut smart_socket = SmartSocket::new(socket_name);
    smart_socket.print_socket_name();
    smart_socket.print_current_power_consumption();
    smart_socket.switch(PowerMode::On);

    let smart_thermometer = SmartThermometer::new();
    smart_thermometer.print_current_temperature();
}
