use esp_idf_svc::hal::{
    gpio::OutputPin,
    ledc::{config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver, Resolution},
    peripheral::Peripheral,
    prelude::*,
    sys::EspError,
};
use std::{thread::sleep, time::Duration};

pub struct ESC<'a> {
    pwm_driver: LedcDriver<'a>,
    timer_conf: TimerConfig,
}

impl<'a> ESC<'a> {
    // TODO: async setup
    pub fn setup<P: OutputPin, T: LedcTimer, C: LedcChannel>(
        pin: &'a mut impl Peripheral<P = P>,
        timer: &'a mut impl Peripheral<P = T>,
        chan: &'a mut impl Peripheral<P = C>,
    ) -> Result<Self, EspError> {
        let timer_conf = TimerConfig::default()
            .frequency(50.Hz().into())
            .resolution(Resolution::Bits14);
        let td = LedcTimerDriver::new(timer, &timer_conf)?;
        let pwm_driver = LedcDriver::new(chan, td, pin)?;

        let mut me = Self {
            timer_conf,
            pwm_driver,
        };
        me.set_speed(0)?;
        me.tween_to(100, Duration::from_secs(2))?;
        sleep(Duration::from_secs(1));
        me.tween_to(0, Duration::from_secs(3))?;
        sleep(Duration::from_secs(1));

        Ok(me)
    }

    pub fn set_speed(&mut self, percent: u32) -> Result<(), EspError> {
        let unit = self.timer_conf.resolution.max_duty() / 20;
        let d = unit + (percent * unit) / 100;
        self.pwm_driver.set_duty(d)?;
        Ok(())
    }

    pub fn get_speed(&self) -> u32 {
        let duty = self.pwm_driver.get_duty();
        let unit = self.timer_conf.resolution.max_duty() / 20;
        return (duty - unit) * 100 / unit;
    }

    //---------------------------------------------------------------------------------------------
    /// Smoothly transition ESC's speed from current value to target percentage.
    //---------------------------------------------------------------------------------------------
    pub fn tween_to(&mut self, percent: u32, d: Duration) -> Result<(), EspError> {
        let cur = self.get_speed();
        let delta = percent.abs_diff(cur);
        let step_duration = if delta == 0 {
            Duration::from_millis(0)
        } else {
            d / delta
        };
        // TODO: remove box dyn indirection
        let rng: Box<dyn Iterator<Item = u32>> = if percent > cur {
            Box::new(cur..=percent)
        } else {
            Box::new((percent..=cur).rev())
        };
        for setpoint in rng {
            sleep(step_duration);
            self.set_speed(setpoint)?;
        }
        Ok(())
    }
}
