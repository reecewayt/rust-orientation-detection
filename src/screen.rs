//@file: src/screen.rs

//todo: 
use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use microbit::{
    display::nonblocking::{BitImage, Display}, 
    pac::{self, interrupt, TIMER1}  
};
use crate::orientation::Orientation;

// static state for the display
static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));
static CURRENT_ORIENTATION: Mutex<RefCell<Option<Orientation>>> = Mutex::new(RefCell::new(None));

// Display patterns
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

pub struct ScreenManager; 

impl ScreenManager {
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

    pub fn clear(&self) {
        cortex_m::interrupt::free(|cs| {
            if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                display.clear();
            }                    
        });
    }
}

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
