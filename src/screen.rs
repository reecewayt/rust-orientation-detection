//! LED display management module for BBC Microbit V2.
//!
//! This module manages the 5x5 LED matrix display on the Microbit, providing
//! visual feedback of the device's current orientation. It uses non-blocking display
//! operations with a 15ms refresh rate to maintain a constant lighting.
//!
//! Sources: https://github.com/nrf-rs/microbit/tree/main/examples/display-nonblocking
//!
//! The display patterns are:
//! - Portrait/PortraitUpsideDown: Vertical bars
//! - LandscapeLeft/LandscapeRight: Horizontal bars
//! - FaceUp/FaceDown: X pattern

use crate::orientation::Orientation;
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::{
    display::nonblocking::{BitImage, Display},
    pac::{self, interrupt, TIMER1},
};

// Global display state and current orientation
// See example: https://github.com/nrf-rs/microbit/tree/main/examples/display-nonblocking
static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));
static CURRENT_ORIENTATION: Mutex<RefCell<Option<Orientation>>> = Mutex::new(RefCell::new(None));

// Display patterns, use bit image as we aren't concerned with brightness
const LANDSCAPE_IMG: BitImage = BitImage::new(&[
    [0, 0, 0, 0, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 0, 0, 0, 0],
]);

const PORTRAIT_IMG: BitImage = BitImage::new(&[
    [0, 1, 1, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 1, 1, 0],
]);

const FACE_UP_AND_DOWN_IMG: BitImage = BitImage::new(&[
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 1, 0],
    [1, 0, 0, 0, 1],
]);

/// Manages the Microbit's 5x5 LED display
pub struct ScreenManager;

impl ScreenManager {
    /// Creates a new ScreenManager with the given timer and display pins.
    ///
    /// # Arguments
    /// * `timer` - TIMER1 peripheral for display refresh timing
    /// * `display_pins` - GPIO pins connected to the LED matrix
    pub fn new(timer: TIMER1, display_pins: microbit::gpio::DisplayPins) -> Self {
        let display = Display::new(timer, display_pins);
        cortex_m::interrupt::free(|cs| {
            *DISPLAY.borrow(cs).borrow_mut() = Some(display);
            *CURRENT_ORIENTATION.borrow(cs).borrow_mut() = Some(Orientation::Portrait);
        });

        // Enable the TIMER1 interrupt
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER1);
        }

        Self
    }
    /// Updates the display pattern based on a new orientation.
    pub fn update(&self, new_orientation: Orientation) {
        cortex_m::interrupt::free(|cs| {
            if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                let image = match new_orientation {
                    Orientation::Portrait => &PORTRAIT_IMG,
                    Orientation::PortraitUpsideDown => &PORTRAIT_IMG,
                    Orientation::LandscapeLeft => &LANDSCAPE_IMG,
                    Orientation::LandscapeRight => &LANDSCAPE_IMG,
                    Orientation::FaceUp => &FACE_UP_AND_DOWN_IMG,
                    Orientation::FaceDown => &FACE_UP_AND_DOWN_IMG,
                };
                display.show(image);
            }
        });
    }

    /// Clears all LEDs on the display.
    ///
    /// Used when entering low power mode or resetting the display state.
    pub fn clear(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                display.clear();
            }
        });
    }
}

/// TIMER1 interrupt handler for display refresh.
///
/// It uses the non-blocking display module's handle_display_event()
/// to efficiently manage the LED matrix multiplexing.
#[interrupt]
fn TIMER1() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            // Handles periodic refesh of the display
            // According to documentation this should be called every 15ms
            display.handle_display_event();
        }
    });
}
