#![allow(dead_code)]

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PowerState {
    Active,
    LowPower,
    Sleep,
}

pub struct PowerManager {
    current_state: PowerState,
    inactivity_timer: u32,
    low_power_threshold: u32,
    sleep_threshold: u32,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            current_state: PowerState::Active,
            inactivity_timer: 0,
            low_power_threshold: 5000,
            sleep_threshold: 30000,
        }
    }

    pub fn update(&mut self, current_time: u32, is_data_stale: bool) -> PowerState {
        if !is_data_stale {
            self.inactivity_timer = current_time;
            if self.current_state != PowerState::Active {
                self.current_state = PowerState::Active;
                return PowerState::Active;
            }
        } else {
            let inactive_duration = current_time.saturating_sub(self.inactivity_timer);
            
            match inactive_duration {
                d if d > self.sleep_threshold => {
                    if self.current_state != PowerState::Sleep {
                        self.current_state = PowerState::Sleep;
                        return PowerState::Sleep;
                    }
                }
                d if d > self.low_power_threshold => {
                    if self.current_state != PowerState::LowPower {
                        self.current_state = PowerState::LowPower;
                        return PowerState::LowPower;
                    }
                }
                _ => {}
            }
        }
        self.current_state
    }
}

