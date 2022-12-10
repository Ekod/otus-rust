#[derive(Debug)]
pub enum PowerMode {
    On,
    Off,
}

pub struct SmartSocket {
    pub name: String,
    pub state: PowerMode,
    power_consumption: u16,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        Self {
            name,
            state: PowerMode::Off,
            power_consumption: 220,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn print_socket_name(&self) {
        println!("The name of the socket is {}", self.name)
    }

    pub fn switch(&mut self, mode: PowerMode) {
        self.state = mode;
    }

    pub fn print_current_power_consumption(&self) {
        println!(
            "The current power consumption is {} watt",
            self.power_consumption
        )
    }
}
