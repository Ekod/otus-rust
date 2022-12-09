pub struct SmartThermometer {
    pub name: String,
    pub current_temperature: u8,
}

impl SmartThermometer {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_temperature: 25,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn print_current_temperature(&self) {
        let celsius_unicode = "\u{2103}";
        println!(
            "The current temperature is {}{celsius_unicode}",
            self.current_temperature
        )
    }
}
