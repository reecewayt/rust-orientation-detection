#![allow(dead_code)] // Disable warnings for unused code for now, FIXME later

// src/orientation.rs

// Import rprintln for debug output
use rtt_target::rprintln;

#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Copy, Clone)] // Add copy and clone traits so we can pass by value to application
pub enum Orientation {
    Portrait,
    PortraitUpsideDown,
    LandscapeLeft,
    LandscapeRight,
    FaceUp,
    FaceDown,
}

pub struct OrientationManager {
    current_orientation: Orientation,
    // Low-pass filter current values
    filtered_x: f32,
    filtered_y: f32,
    filtered_z: f32,
    alpha: f32,
    last_update: i32,
}

impl OrientationManager {
    pub fn new() -> Self {
        Self {
            current_orientation: Orientation::Portrait,
            filtered_x: 0.0,
            filtered_y: 0.0,
            filtered_z: 0.0,
            alpha: 0.05, // Default filter coefficient
            last_update: 0,
            //stable_duration: 0,
            //stability_threshold: 1000, // TODO: add stability to orientation changes as well
        }
    }

    fn abs(x: f32) -> f32 {
        if x < 0.0 { -x } else { x }
    }

    #[cfg(feature = "filter-debug")]
    fn print_debug(&mut self, raw_x: f32, raw_y: f32, raw_z: f32) {
        // Print raw values in mg and filtered values converted back to mg
        rprintln!("{},{},{},{},{},{}", 
            (raw_x * 1000.0) as i32, 
            (raw_y * 1000.0) as i32, 
            (raw_z * 1000.0) as i32,
            (self.filtered_x * 1000.0) as i32,
            (self.filtered_y * 1000.0) as i32,
            (self.filtered_z * 1000.0) as i32
        );
    }

    pub fn process_sample(&mut self, x: i32, y: i32, z: i32, timestamp: i32) -> Option<Orientation> {
        // Convert mg to g
        let x_g = x as f32 / 1000.0;
        let y_g = y as f32 / 1000.0;
        let z_g = z as f32 / 1000.0;

        // Simple first-order low pass filter
        // Source: https://dobrian.github.io/cmp/topics/filters/lowpassfilter.html
        // Our goal here is to smooth out the data to reduce noise and jitter
        // Hence, we are not interested in the AC component of the acceleration but
        // only the DC (or static) component. 
        self.filtered_x = self.alpha * x_g + (1.0 - self.alpha) * self.filtered_x;
        self.filtered_y = self.alpha * y_g + (1.0 - self.alpha) * self.filtered_y;
        self.filtered_z = self.alpha * z_g + (1.0 - self.alpha) * self.filtered_z;

        
        #[cfg(feature = "filter-debug")]
        self.print_debug(x_g, y_g, z_g);

        self.last_update = timestamp;

        // Determine orientation from filtered values
        const THRESHOLD: f32 = 0.8;
        
        let new_orientation = if Self::abs(self.filtered_x) > THRESHOLD {
            if self.filtered_x > 0.0 {
                Orientation::Portrait
            } else {
                Orientation::PortraitUpsideDown
            }
        } else if Self::abs(self.filtered_y) > THRESHOLD {
            if self.filtered_y > 0.0 {
                Orientation::LandscapeLeft
            } else {
                Orientation::LandscapeRight
            }
        } else if Self::abs(self.filtered_z) > THRESHOLD {
            if self.filtered_z > 0.0 {
                Orientation::FaceUp
            } else {
                Orientation::FaceDown
            }
        } else {
            return None;
        };

        // Only report changes in orientation
        if new_orientation != self.current_orientation {
            self.current_orientation = new_orientation;
            Some(self.current_orientation)
        } else {
            None
        }
    }


    // Application code can call this to set the filter coefficient
    // Default value is 0.1 
    pub fn set_filter_coefficient(&mut self, alpha: f32) {
        assert!(alpha >= 0.0 && alpha <= 1.0, "Alpha must be between 0 and 1");
        self.alpha = alpha;
    }

}
