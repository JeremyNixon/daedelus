mod esc;
mod servo;
mod sim7000g;
use esc::ESC;
use esp_idf_svc::hal::{peripherals::Peripherals, prelude::*};
use servo::Servo;
use std::{thread::sleep, time::Duration};

/*
use esp_idf_svc::hal::{
    delay::Ets,
    i2c::{I2cConfig, I2cDriver},
};
use mpu9250::Mpu9250;
*/

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let mut peripherals = Peripherals::take()?;

    /*
    {
        let d = I2cDriver::new(
            peripherals.i2c0,
            peripherals.pins.gpio21,
            peripherals.pins.gpio22,
            &I2cConfig::new()
                .baudrate(40.kHz().into())
                .sda_enable_pullup(false)
                .scl_enable_pullup(false),
        )?;

        let mut ac = Mpu9250::marg_default(d, &mut Ets).expect(
            "Failed to initialize IMU");
        for _ in 1..5 {
            let a = ac.all::<[f32; 3]>().unwrap();
            println!("{:?}", a);
        }
    }
    */

    let mut left_elevator = Servo::new(
        &mut peripherals.pins.gpio13,
        &mut peripherals.ledc.timer2,
        &mut peripherals.ledc.channel2,
    )?;
    let mut right_elevator = Servo::new(
        &mut peripherals.pins.gpio12,
        &mut peripherals.ledc.timer3,
        &mut peripherals.ledc.channel3,
    )?;

    sleep(Duration::from_secs(2)); // gives escs time to prepare
    let mut left_motor = ESC::setup(
        &mut peripherals.pins.gpio32,
        &mut peripherals.ledc.timer0,
        &mut peripherals.ledc.channel0,
    )?;
    let mut right_motor = ESC::setup(
        &mut peripherals.pins.gpio33,
        &mut peripherals.ledc.timer1,
        &mut peripherals.ledc.channel1,
    )?;
    sleep(Duration::from_secs(2));

    left_motor.set_speed(100);
    sleep(Duration::from_secs(5));
    left_motor.set_speed(0);
    Ok(())

    /*
    loop {
        left_elevator.set_pos(0)?;
        right_elevator.set_pos(100)?;
        sleep(Duration::from_secs(5));
        left_elevator.set_pos(100)?;
        right_elevator.set_pos(0)?;
        sleep(Duration::from_secs(5));
    }
    */
}
