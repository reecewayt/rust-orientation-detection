#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod orientation;
mod power;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::{
    hal::{
        prelude::*,
        timer::Timer,
        twim,
    },
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{
    AccelOutputDataRate, Lsm303agr,
};

use crate::orientation::OrientationManager;
use crate::power::PowerManager;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0); //TODO: Implement timer interrupt
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // Code from documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
    
    let mut orientation_manager = OrientationManager::new();
    
    loop {
        let current_time = timer.read();
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            // RTT instead of normal print
            //rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);

            if let Some(new_orientation) = orientation_manager.process_sample(
                data.x, 
                data.y, 
                data.z, 
                current_time as i32
            ) {
                rprintln!("New orientation: {:?}", new_orientation);
            }
        }
    }
}
