pub enum PowerMode {
    On,
    Off,
}

pub struct SmartSocket {
    name: String,
    is_enabled: PowerMode,
    power_consumption: u16,
}

impl SmartSocket {
    pub fn new(name: String) -> Self {
        Self {
            name,
            is_enabled: PowerMode::Off,
            power_consumption: 220,
        }
    }

    pub fn description(&self) {
        println!("The name of the socket is {}", self.name)
    }

    pub fn switch(&mut self, mode: PowerMode) {
        self.is_enabled = mode;
    }

    pub fn get_current_power_consumption(&self) {
        println!("The current power consumption is {} watt", self.power_consumption)
    }
}