#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod orientation;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::hal::prelude::*;

use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr,
};

use crate::orientation::OrientationManager;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    
    let mut orientation_manager = OrientationManager::new();
    
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            // RTT instead of normal print
            //rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
            let x = data.x as f32 / 1000.0; 
            let y = data.y as f32 / 1000.0;
            let z = data.z as f32 / 1000.0;
            if let Some(new_orientation) = orientation_manager.update(x, y, z) {
                rprintln!("New orientation: {:?}", new_orientation);
            }
        }
    }
}
