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
    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            // RTT instead of normal print
            rprintln!("Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
    }
}


/*
#![deny(unsafe_code)]
#![no_main]
#![no_std]

mod accelerometer;
mod orientation;
mod state;

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

use crate::accelerometer::AccelerometerManager;
use crate::orientation::OrientationManager;
use crate::state::StateManager;

#[entry]
fn main() -> ! {
    // Your existing initialization code...
    
    let mut accel_manager = AccelerometerManager::new();
    let mut orientation_manager = OrientationManager::new();
    let mut state_manager = StateManager::new();

    loop {
        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            
            // Update managers
            accel_manager.add_sample(data.x as f32, data.y as f32, data.z as f32, 0);
            let filtered_data = accel_manager.get_filtered_values();
            
            if let Some(new_orientation) = orientation_manager.update(
                filtered_data.x, 
                filtered_data.y, 
                filtered_data.z
            ) {
                rprintln!("New orientation: {:?}", new_orientation);
            }

            if let Some(new_state) = state_manager.update(
                (filtered_data.x.powi(2) + 
                 filtered_data.y.powi(2) + 
                 filtered_data.z.powi(2)).sqrt()
            ) {
                rprintln!("Device state changed: {:?}", new_state);
            }
        }
    }
}
*/