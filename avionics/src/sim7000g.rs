use esp_idf_svc::hal::uart::UartDriver;

pub struct SIM7000g<'a> {
    uart: &'a mut UartDriver<'a>,
}

impl<'a> SIM7000g<'a> {
    pub fn new(uart: &'a mut UartDriver<'a>) -> Self {
        Self { uart }
    }
}
