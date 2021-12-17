use linux_embedded_hal::{Delay, I2cdev};
use bme280::BME280;
use ltr_559::{Ltr559, SlaveAddr, AlsGain, AlsIntTime, AlsMeasRate};
use alsa::{Direction, ValueOr, Error};
use alsa::pcm::{PCM, HwParams, Format, Access, State};
use std::ffi::{CString, CStr};
use std::io::{stdout, Write, BufWriter};
use std::thread::sleep;
use std::time::Duration;

use rppal::gpio::Gpio;

fn main() {
    let devicename = CString::new("plughw:0").unwrap();
    let pcm = PCM::open(&devicename, Direction::Capture, false).unwrap();

    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(48000, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s32()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();

    let io = pcm.io_i32().unwrap();

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
    pcm.sw_params(&swp).unwrap();

    

    loop {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());
        writeln!(out, "\x1B[2J");
        writeln!(out, "\x1B[0;0H");
        let status = sensor.get_status().unwrap();
        if status.als_data_valid {
            let (lux_raw_0, lux_raw_1) = sensor.get_als_raw_data().unwrap();
            let lux = sensor.get_lux().unwrap();
            writeln!(out, "Lux = {}", lux);
        }
        let measurements = bme280.measure().unwrap();
        writeln!(out, 
            "Humidity = {}%, Temperature = {}degC, Pressure = {}hpa",
            measurements.humidity, measurements.temperature, (measurements.pressure / 100.00)
        );

        {
            let mut soundbuf = [0i32; 8192];
            let mut ret: Result<usize, Error> = Result::Ok(0);
            if pcm.state() != State::Running {
                match pcm.start() {
                    Ok(())  => {
                        println!("start pcm mic.");
                        ret = io.readi(&mut soundbuf[..]);
                    },
                    Err(err) => println!("error start pcm mic.{:?}", err),
                };
            } else {
                ret = io.readi(&mut soundbuf[..]);
            }

            match ret {
                Ok(size) => {
                    // writeln!(out, "read size is {}", size);
                    let mut prev = 0;
                    let mut preprev = 0;
                    let mut peak = [0;2];
                    let mut maxamp = 0;
                    for amplitude in soundbuf.iter() {
                        if prev > preprev && prev > *amplitude {
                            peak[0] = prev;
                        } else if prev < preprev && prev < *amplitude {
                            peak[1] = prev;
                        }
                        if !(peak[0] == 0 || peak[1] == 0) {
                            if maxamp < (peak[0] - peak[1]) {
                                maxamp = peak[0] - peak[1];
                            }
                        }
                        preprev = prev;
                        prev = *amplitude;
                    }
                    writeln!(out, "sound amplitude: {}", maxamp.abs() / 1000000);
                },
                Err(err) => {
                    println!("no input.{:?}", err);
                },
            };
        }
        sleep(Duration::from_millis(1));
    }
}