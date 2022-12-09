mod smart_devices;

use crate::smart_devices::smart_house::SmartHouse;
use smart_devices::smart_house::Room;
use smart_devices::smart_socket::SmartSocket;
use smart_devices::smart_thermometer::SmartThermometer;

fn main() {
    let socket_name = String::from("smart socket");
    let smart_socket = SmartSocket::new(socket_name.clone());

    let thermometer_name = String::from("smart thermometer");
    let smart_thermometer = SmartThermometer::new(thermometer_name.clone());

    let living_room_name = String::from("living room");
    let mut living_room = Room::new(living_room_name);

    living_room.add_device_name(smart_socket.get_name());
    living_room.add_device_name(smart_thermometer.get_name());

    let house_name = String::from("smart house");
    let mut house = SmartHouse::new(house_name);

    house.add_room(living_room);

    let devices = house.devices(&living_room);
    match devices {
        None => println!("No devices for provided room found or room doesn't exist"),
        Some(devices_list) => println!("{:?}", devices_list),
    }
}
