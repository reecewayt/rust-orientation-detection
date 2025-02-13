
use microbit::hal::Timer;
use libm::{sqrt, fabs};  // Add this import
use rtt_target::rprintln; 

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PowerState {
    Active,     // Active mode
    LowPower,   // Low power mode
}

pub struct PowerManager {
    movement_threshold: i32, 
    inactivity_threshold: u32,
    last_movement_time: u32,
    current_state: PowerState
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            movement_threshold: 75,        // 75 mg
            inactivity_threshold: 10 * Timer::<microbit::pac::TIMER0>::TICKS_PER_SECOND, // 10 seconds
            last_movement_time: 0,
            current_state: PowerState::Active,            
        }
    }

    pub fn process_accel_data(&mut self, x: i32, y: i32, z: i32, current_time: u32) -> Option<PowerState> {
        // Take euclidean norm of the acceleration vector to determine magnitude
        let magnitude = sqrt((x * x + y * y + z * z)as f64);
        // Sub 1000mg from magnitude to get acceleration relative to gravity
        // When device is stationary, this should be close to 0
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
            rprintln!("Power state transition: {:?} -> {:?}", self.current_state, new_state);
            self.current_state = new_state;
            Some(self.current_state)
        } else {
            None
        }
    }

    pub fn get_state(&self) -> PowerState {
        self.current_state
    }

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