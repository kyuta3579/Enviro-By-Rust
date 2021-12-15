use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use ltr_559::{Ltr559, SlaveAddr, AlsGain, AlsIntTime, AlsMeasRate};
use alsa::{Direction, ValueOr};
use alsa::pcm::{PCM, HwParams, Format, Access, State};
use spectrum_analyzer::{samples_fft_to_spectrum, FrequencyLimit};
use std::error::Error;
use rppal::gpio::Gpio;

fn main() {

    let pcm = PCM::new("plughw:0", Direction::Capture, true).unwrap();

    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(48000, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::float()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();

    let io = pcm.io_f32().unwrap();

    let i2c_bme280 = I2cdev::new("/dev/i2c-1").unwrap();
    let mut bme280 = BME280::new_primary(i2c_bme280, Delay);

    let i2c_ltr559 = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let mut sensor = Ltr559::new_device(i2c_ltr559, address);

    bme280.init().unwrap();
    sensor
          .set_als_meas_rate(AlsIntTime::_50ms, AlsMeasRate::_50ms)
          .unwrap();
       sensor.set_als_contr(AlsGain::Gain4x, false, true).unwrap();
    
    let hwp = pcm.hw_params_current().unwrap();
    let swp = pcm.sw_params_current().unwrap();
    swp.set_start_threshold(hwp.get_buffer_size().unwrap()).unwrap();
    pcm.sw_params(&swp).unwrap();

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

        let mut soundbuf = [0f32; 1024];
        let ret = io.readi(&mut soundbuf[..]);

        if match ret
        let res = samples_fft_to_spectrum(
            &soundbuf[..],
            48000,
            FrequencyLimit::All,
            Some(&|val, info| val - info.min),
        );
    }
}