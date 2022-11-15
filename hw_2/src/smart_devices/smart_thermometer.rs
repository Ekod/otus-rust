pub struct SmartThermometer {
    current_temperature: u8,
}

impl SmartThermometer {
    pub fn new() -> Self {
        Self {
            current_temperature: 25,
        }
    }

    pub  fn get_current_temperature(&self) {
        let celsius_unicode = "\u{2103}";
        println!("The current temperature is {}{celsius_unicode}", self.current_temperature)
    }
}
