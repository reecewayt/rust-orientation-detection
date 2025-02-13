//! Orientation detection module for BBC Microbit V2.
//!
//! This module provides orientation detection capabilities by processing accelerometer data
//! to determine the device's orientation relative to the ground. It implements a low-pass filter
//! to extract the DC component of the gravity acceleration vector- attenuating against
//! hand movements and device shaking.
//!
//! # Example
//! ```
//! let mut orientation_manager = OrientationManager::new();
//! if let Some(new_orientation) = orientation_manager.process_sample(x, y, z) {
//!     println!("New orientation detected: {:?}", new_orientation);
//! }
//! ```

#[cfg(feature = "filter-debug")]
use rtt_target::rprintln;

use libm::fabsf;

/// Represents the possible orientations of the Microbit device.
///
/// The orientation is determined by the predominant direction of the gravity vector
/// as measured by the accelerometer.
#[cfg_attr(debug_assertions, derive(Debug))]
#[derive(PartialEq, Copy, Clone)]
pub enum Orientation {
    Portrait,
    PortraitUpsideDown,
    LandscapeLeft,
    LandscapeRight,
    FaceUp,
    FaceDown,
}

/// Manages device orientation state and filtering
pub struct OrientationManager {
    current_orientation: Orientation,
    // Low-pass filtered current values
    filtered_x: f32,
    filtered_y: f32,
    filtered_z: f32,
    // Filter coefficient controlling smoothing (0.0 = no smoothing, 1.0 = max smoothing)
    alpha: f32,
}

impl OrientationManager {
    /// Initializes with:
    /// - Portrait orientation
    /// - Zero initial filter values
    /// - Default filter coefficient (alpha) of 0.05
    pub fn new() -> Self {
        Self {
            current_orientation: Orientation::Portrait,
            filtered_x: 0.0,
            filtered_y: 0.0,
            filtered_z: 0.0,
            alpha: 0.05, // Default filter coefficient
        }
    }

    /// Prints debug information when the filter-debug feature is enabled.
    ///
    /// Outputs raw and filtered accelerometer values in CSV format for analysis.
    #[cfg(feature = "filter-debug")]
    fn print_debug(&mut self, raw_x: f32, raw_y: f32, raw_z: f32) {
        // Print raw values in mg and filtered values converted back to mg
        rprintln!(
            "{},{},{},{},{},{}",
            (raw_x * 1000.0) as i32,
            (raw_y * 1000.0) as i32,
            (raw_z * 1000.0) as i32,
            (self.filtered_x * 1000.0) as i32,
            (self.filtered_y * 1000.0) as i32,
            (self.filtered_z * 1000.0) as i32
        );
    }

    /// Processes new accelerometer samples to determine device orientation.
    ///
    /// # Arguments
    /// * `x` - X-axis acceleration in milligravities (mg)
    /// * `y` - Y-axis acceleration in milligravities (mg)
    /// * `z` - Z-axis acceleration in milligravities (mg)
    ///
    /// # Returns
    /// * `Some(Orientation)` if orientation has changed
    /// * `None` if orientation remains the same or is indeterminate
    pub fn process_sample(&mut self, x: i32, y: i32, z: i32) -> Option<Orientation> {
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

        //self.last_update = timestamp;

        // Determine orientation from filtered values
        const THRESHOLD: f32 = 0.8;

        let new_orientation = if fabsf(self.filtered_x) > THRESHOLD {
            if self.filtered_x > 0.0 {
                Orientation::Portrait
            } else {
                Orientation::PortraitUpsideDown
            }
        } else if fabsf(self.filtered_y) > THRESHOLD {
            if self.filtered_y > 0.0 {
                Orientation::LandscapeLeft
            } else {
                Orientation::LandscapeRight
            }
        } else if fabsf(self.filtered_z) > THRESHOLD {
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

    /// Sets the low-pass filter coefficient.
    #[allow(dead_code)]
    pub fn set_filter_coefficient(&mut self, alpha: f32) {
        assert!(
            (0.0..=1.0).contains(&alpha),
            "Alpha must be between 0 and 1"
        );
        self.alpha = alpha;
    }

    /// Getter method that returns the current orientation of the device.
    #[allow(dead_code)]
    pub fn get_orientation(&self) -> Orientation {
        self.current_orientation
    }
}
