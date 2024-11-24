use esp_idf_svc::hal::{
    gpio::OutputPin,
    ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver, Resolution},
    peripheral::Peripheral,
    prelude::*,
    sys::EspError,
};
pub struct Servo<'a> {
    pwm_driver: LedcDriver<'a>,
    timer_conf: TimerConfig,
}

impl<'a> Servo<'a> {
    // TODO: async setup
    pub fn new<P: OutputPin, T: LedcTimer, C: LedcChannel>(
        pin: &'a mut impl Peripheral<P = P>,
        timer: &'a mut impl Peripheral<P = T>,
        chan: &'a mut impl Peripheral<P = C>,
    ) -> Result<Self, EspError> {
        let timer_conf = TimerConfig::default()
            .frequency(50.Hz().into())
            .resolution(Resolution::Bits14);
        let td = LedcTimerDriver::new(timer, &timer_conf)?;
        let pwm_driver = LedcDriver::new(chan, td, pin)?;

        Ok(Self {
            timer_conf,
            pwm_driver,
        })
    }

    pub fn set_pos(&mut self, percent: u32) -> Result<(), EspError> {
        let unit = self.timer_conf.resolution.max_duty() / 20;
        let d = unit + (percent * unit) / 100;
        self.pwm_driver.set_duty(d)?;
        Ok(())
    }

    pub fn get_pos(&self) -> u32 {
        let duty = self.pwm_driver.get_duty();
        let unit = self.timer_conf.resolution.max_duty() / 20;
        return (duty - unit) * 100 / unit;
    }
}