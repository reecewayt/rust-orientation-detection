//! BBC Microbit V2 Orientation Detection Application
//!
//! A smartphone-style orientation detection system that displays device orientation
//! using the Microbit's 5x5 LED matrix. Features include:
//!
//! - Real-time orientation detection using the LSM303AGR accelerometer
//! - Low-pass filtered acceleration data for stable orientation detection
//! - Power management with automatic low-power mode during inactivity
//! - Non-blocking LED display updates
//!
//! The application monitors the device's orientation relative to gravity and
//! displays different patterns on the LED matrix to indicate portrait, landscape,
//! or face up/down orientations.

#![no_main]
#![no_std]

mod orientation;
mod power;
mod screen;

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use microbit::{
    hal::{prelude::*, timer::Timer, twim},
    pac::twim0::frequency::FREQUENCY_A,
};

use lsm303agr::{AccelMode, AccelOutputDataRate, Lsm303agr};

use crate::orientation::OrientationManager;
use crate::power::{PowerManager, PowerState};
use crate::screen::ScreenManager;

/// Main entry point
/// Initializes peripherals (timer, I2C, accelerometer), and constructs orientation, screen, and power managers
/// Main loop continuously polls accelerometer for new data
#[entry]
fn main() -> ! {
    // RTT (Real-Time Transfer) initialization for debugging console
    rtt_init_print!();
    // Gets board peripherals
    let board = microbit::Board::take().unwrap();
    // Initialize I2C interface
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    // Initialize screen manager with TIMER1
    let screen_manager = ScreenManager::new(board.TIMER1, board.display_pins);

    // Initialize accelerometer
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    // Initialize orientation and power managers
    let mut orientation_manager = OrientationManager::new();
    let mut power_manager = PowerManager::new();
    let mut timer = Timer::new(board.TIMER0);

    #[cfg(debug_assertions)]
    rprintln!("Initialization complete, entering main loop");

    // Start the timer for power management timing
    timer.start(0xFFFF_FFFFu32);

    loop {
        if !sensor.accel_status().unwrap().xyz_new_data {
            continue; // Skip this iteration if no new data is available
        }

        let data = sensor.accel_data().unwrap();
        let current_time = timer.read();

        // Handle power state changes
        if let Some(power_state) =
            power_manager.process_accel_data(data.x, data.y, data.z, current_time)
        {
            match power_state {
                PowerState::LowPower => {
                    // Enter low power mode: clear display and set accelerometer to low power mode with low ODR
                    screen_manager.clear();
                    sensor.set_accel_mode(AccelMode::LowPower).unwrap();
                    sensor.set_accel_odr(AccelOutputDataRate::Hz1).unwrap();

                    #[cfg(debug_assertions)]
                    rprintln!("Entering low power mode");
                }
                PowerState::Active => {
                    // Resume normal operation: set accelerometer to normal mode with higher ODR
                    sensor.set_accel_mode(AccelMode::Normal).unwrap();
                    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

                    #[cfg(debug_assertions)]
                    rprintln!("Resuming normal operation");
                }
            }
        }

        // Update orientation display if in active mode
        if power_manager.get_state() == PowerState::Active {
            if let Some(new_orientation) =
                orientation_manager.process_sample(data.x, data.y, data.z)
            {
                #[cfg(debug_assertions)]
                rprintln!("New orientation detected: {:?}", new_orientation);
                screen_manager.update(new_orientation);
            }
        }
    }
}
