//@file: src/main.rs

#![no_main]
#![no_std]

mod orientation;
mod power;
mod screen; 

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
    AccelOutputDataRate, 
    Lsm303agr,
    AccelMode,
};

use crate::orientation::OrientationManager;
use crate::power::{PowerManager, PowerState};
use crate::screen::ScreenManager;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
   
    // Initialize screen manager with TIMER1
    let screen_manager = ScreenManager::new(board.TIMER1, board.display_pins);
    
    // Initialize accelerometer
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
   
    let mut orientation_manager = OrientationManager::new();
    let mut power_manager = PowerManager::new();
    let mut timer = Timer::new(board.TIMER0);

    #[cfg(debug_assertions)]
    rprintln!("Initialization complete, entering main loop");

    // Start the timer, see documentation for Timer::start for more information
    timer.start(0xFFFF_FFFFu32); 
   
    loop {
        if !sensor.accel_status().unwrap().xyz_new_data {
            continue; // Skip this iteration if no new data is available
        }

        let data = sensor.accel_data().unwrap();
        let current_time = timer.read();

        // Handle power state changes
        if let Some(power_state) = power_manager.process_accel_data(
            data.x,
            data.y,
            data.z,
            current_time
        ) {
            match power_state {
                PowerState::LowPower => {
                    screen_manager.clear();
                    sensor.set_accel_mode(AccelMode::LowPower).unwrap();
                    sensor.set_accel_odr(AccelOutputDataRate::Hz1).unwrap();
                    
                    #[cfg(debug_assertions)]
                    rprintln!("Entering low power mode");
                }
                PowerState::Active => {
                    sensor.set_accel_mode(AccelMode::Normal).unwrap();
                    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();
                    
                    #[cfg(debug_assertions)]
                    rprintln!("Resuming normal operation");
                }
            }
        }

        // Process orientation updates only in active mode
        if power_manager.get_state() == PowerState::Active {
            if let Some(new_orientation) = orientation_manager.process_sample(
                data.x,
                data.y,
                data.z,
            ) {
                #[cfg(debug_assertions)]
                rprintln!("New orientation detected: {:?}", new_orientation);
                screen_manager.update(new_orientation);
            }
        }
    }
}
