//! Power management module for BBC Microbit V2.
//!
//! This module manages power states by monitoring device movement through accelerometer data.
//! It implements inactivity detection that transitions the device between Active
//! and LowPower states based on the magnitude of movement and time since last
//! motion.
//!
//! The module uses the Euclidean norm of acceleration vectors relative to gravity (1g)
//! to detect movement. When movement falls below a threshold for a specified duration,
//! the device enters low power mode to conserve energy.

use libm::{fabs, sqrt};
use microbit::hal::Timer;

#[cfg(feature = "power-debug")]
use rtt_target::rprintln;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PowerState {
    Active,   // Active mode
    LowPower, // Low power mode
}

/// Manages device power states based on movement detection
pub struct PowerManager {
    /// Threshold in mg (milligravity) for detecting significant enough movement
    movement_threshold: i32,
    /// Time in ticks for inactivity before entering low power mode
    inactivity_threshold: u32,
    /// Timer value when last movement was detected
    last_movement_time: u32,
    /// Holds current state of the power manager
    current_state: PowerState,
}

impl PowerManager {
    /// Creates a new PowerManager with default settings.
    ///
    /// Initializes with:
    /// - Movement threshold: 75 mg
    /// - Inactivity threshold: 10 seconds
    /// - Starting in Active state
    pub fn new() -> Self {
        Self {
            movement_threshold: 75,
            inactivity_threshold: 10 * Timer::<microbit::pac::TIMER0>::TICKS_PER_SECOND,
            last_movement_time: 0,
            current_state: PowerState::Active,
        }
    }

    /// Processes accelerometer data to determine appropriate power state.
    ///
    /// # Arguments
    /// * `x` - X-axis acceleration in milligravities (mg)
    /// * `y` - Y-axis acceleration in milligravities (mg)
    /// * `z` - Z-axis acceleration in milligravities (mg)
    /// * `current_time` - Current system time in timer ticks
    ///
    /// # Returns
    /// * `Some(PowerState)` if power state has changed
    /// * `None` if power state remains the same
    ///
    pub fn process_accel_data(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        current_time: u32,
    ) -> Option<PowerState> {
        // Take euclidean norm of the acceleration vector to determine magnitude of the vector
        let magnitude = sqrt((x * x + y * y + z * z) as f64);
        // Sub 1000mg from magnitude to get acceleration relative to gravity
        // When device is stationary, this should be less than movement_threshold
        let movement = fabs(magnitude - 1000.0) as i32;

        #[cfg(feature = "power-debug")]
        self.print_debug(movement, magnitude, current_time);

        let new_state = if movement > self.movement_threshold {
            self.last_movement_time = current_time;
            PowerState::Active
        } else if current_time - self.last_movement_time > self.inactivity_threshold {
            PowerState::LowPower
        } else {
            self.current_state
        };

        if new_state != self.current_state {
            #[cfg(feature = "power-debug")]
            rprintln!(
                "Power state transition: {:?} -> {:?}",
                self.current_state,
                new_state
            );
            self.current_state = new_state;
            Some(self.current_state)
        } else {
            None
        }
    }

    /// Getter method that returns the current power state
    pub fn get_state(&self) -> PowerState {
        self.current_state
    }

    /// Outputs debug information when power-debug feature is enabled.
    /// This is useful when analyzing the power manager's behavior and finding
    /// optimal thresholds.
    #[cfg(feature = "power-debug")]
    fn print_debug(&self, movement: i32, magnitude: f64, current_time: u32) {
        rprintln!(
            "Power Debug: mov={}, mag={:.2}, time={}, last_mov={}, state={:?}",
            movement,
            magnitude,
            current_time,
            self.last_movement_time,
            self.current_state
        );
    }
}
