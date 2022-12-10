use std::collections::{HashMap, HashSet};
use std::io::{Error, ErrorKind};

struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    fn new(name: String) -> Self {
        Self {
            name,
            rooms: HashMap::new(),
        }
    }

    fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    fn devices(&self, room: &str) -> Result<HashSet<String>, Error> {
        let room = self.rooms.get(room);
        match room {
            None => {
                let return_err = Error::new(ErrorKind::NotFound, "room not found");
                Err(return_err)
            }
            Some(&room) => Ok(room.devices),
        }
    }

    fn create_report(&self, info: &impl DeviceInfoProvider) -> String {
        todo!("перебор комнат и устройств в них для составления отчёта")
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: String, device: String) -> String;
}

struct Room {
    name: String,
    devices: HashSet<String>,
}
