#![allow(dead_code)] // Disable warnings for unused code for now, FIXME later

// src/orientation.rs
pub struct AccelData {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    timestamp: i32,
}

//todo
pub struct AccelDataManager {
    samples: [AccelData; 10],  // Circular buffer maybe?
    current_index: usize,
}

impl AccelDataManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add_sample(&mut self, _x: i32, _y: i32, _z: i32, _timestamp: i32) {
        todo!()
    }

    pub fn get_filtered_values(&self) -> AccelData {
       todo!()
       // Might need some form of filtering here to ensure good state
       // transistions
    }
}

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
    stable_duration: u32,       // Time in stable position
    stability_threshold: u32,   // Required time for orientation change
}

impl OrientationManager {
    pub fn new() -> Self {
        Self {
            current_orientation: Orientation::Portrait,
            stable_duration: 0,
            stability_threshold: 1000, // 1 second default
        }
    }

    fn abs(x: f32) -> f32 {
        if x < 0.0 { -x } else { x }
    }

    pub fn update(&mut self, x: f32, y: f32, z: f32) -> Option<Orientation> {
        // Threshold for considering an axis as "primary" direction
        // Using 0.8 as a threshold means the axis needs to experience
        // at least ~80% of gravity's acceleration (1g)
        const THRESHOLD: f32 = 0.8; 

        // Determine orientation based on which axis has strongest acceleration
        let new_orientation = if Self::abs(x) > THRESHOLD {
            if x > 0.0 {
                Orientation::Portrait
            } else {
                Orientation::PortraitUpsideDown
            }
        } else if Self::abs(y) > THRESHOLD {
            if y > 0.0 {
                Orientation::LandscapeLeft
            } else {
                Orientation::LandscapeRight
            }
        } else if Self::abs(z)> THRESHOLD {
            if z > 0.0 {
                Orientation::FaceUp
            } else {
                Orientation::FaceDown
            }
        } else {
            // No orientation change
            return None;
        };

        // Update orientation only on state changes, otherwise stay the same
        if new_orientation != self.current_orientation {
            self.current_orientation = new_orientation;
            Some(self.current_orientation)
        }
        else {
            None
        }

    }
}

pub enum DevicePowerManager {
    Awake,
    Sleep,
}

impl DevicePowerManager {
    pub fn from_orientation(_orientation: &Orientation) -> Self {
        // Implement power changes from state transition logic
        todo!()
    }
}