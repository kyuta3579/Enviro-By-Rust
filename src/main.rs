use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use ltr_559::{Ltr559, SlaveAddr, AlsGain, AlsIntTime, AlsMeasRate};
use std::error::Error;
use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access, State};
use rppal::gpio::Gpio;

fn main() {

    let pcm = PCM::new("sndrpii2scard", Direction::Capture, false).unwrap();

    // using Linux I2C Bus #1 in this example
    let i2c_bme280 = I2cdev::new("/dev/i2c-1").unwrap();

    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c_bme280, Delay);

    // or, initialize the BME280 using the secondary I2C address 0x77
    // let mut bme280 = BME280::new_secondary(i2c_bme280, Delay);

    // or, initialize the BME280 using a custom I2C address
    // let bme280_i2c_addr = 0x88;
    // let mut bme280 = BME280::new(i2c_bme280, bme280_i2c_addr, Delay);

    let i2c_ltr559 = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Ltr559::new_device(i2c_ltr559, address);
    bme280.init().unwrap();
    sensor
          .set_als_meas_rate(AlsIntTime::_50ms, AlsMeasRate::_50ms)
          .unwrap();
       sensor.set_als_contr(AlsGain::Gain4x, false, true).unwrap();


    loop {
        let status = sensor.get_status().unwrap();
        if status.als_data_valid {
            let (lux_raw_0, lux_raw_1) = sensor.get_als_raw_data().unwrap();
            let lux = sensor.get_lux().unwrap();
            println!(
                "Raw Lux CH1: 0x{:04x}, CH0: 0x{:04x} Lux = {}, Status.als_data_valid = {}",
                lux_raw_0, lux_raw_1, lux, status.als_data_valid
            );
        }
        let measurements = bme280.measure().unwrap();
        println!("Relative Humidity = {}%", measurements.humidity);
        println!("Temperature = {} deg C", measurements.temperature);
        println!("Pressure = {} pascals", measurements.pressure);
    }
}