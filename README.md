# Orientation Detection Application
**Description**: This project uses the BBC Microbit V2 to learn about embedded rust and develop a working application for the final project of Rust Programming (CS 523) at Portland State University. What I've done here is implemented a smartphone-style orientation detection binary crate. When running the BBC Microbit will display a screen orientation that match the device orientation relative to the ground. This means we are mostly sensing the DC components of the gravity acceleration vector. 

**Author**: Reece Wayt  
**Date**: 2/13/2025
**License**: This repo is licensed under [APACHE 2.0](./Embed.toml) and [MIT](./LICENSE-MIT.md)


### Application Components
1. 3D Orientation Detection: The orientation manager (`src/orientation.rs`) reads the data from the application and processes the raw values from the accelerometer. To get the DC components of acceleration I used a low pass filter to attenuate shaking and perturbations from a human holding the device. In this sense, the application is somewhat robust against noise from the sensor data and looks at the perdominant DC component of the gravity vector to determine its orientation relative to the ground. 

2. Screen Display: On the Microbit is a 5x5 LED display, the screen manager (`src/screen.rs`) manages states and state changes based on the orientation manager. This module specifically displays and refreshes the screen every 15 ms based on the current state, and it operates in non-blocking mode. Please see the docs regarding the [non-blocking module](https://docs.rs/microbit-v2/0.15.1/microbit/display/nonblocking/index.html)

3. Low Power Mode: The power manager (`src/power.rs`) manages power states based on raw sensor output data to detect if the device is currently being held. It will sense small vibrations and if these are greater than ` movement_threshold` the device will stay in `Active` mode and the screen matrix will stay on. In `LowPower` mode the device will power down the display and change the accelerometer's mode to low power and reduce the sampling rate to 1Hz. The threshold values for this module were tested and found using the feature `power_debug`, see the debugging and testing section below. See the debugging and testing section below for more details on the filter parameters. 

### Running and Building
```bash
# building 
cargo build 
# or 
cargo build --target thumbv7em-none-eabihf
# running 
cargo embed 
# or 
cargo embed --target thumbv7em-none-eabihf
```
I've included a `.cargo/config.toml` file with this binary crate so you can omit the --target when invoking cargo. 

```
# Example Application Output (Debug Build)
cargo embed

# Output
19:23:15.426: Initialization complete, entering main loop
19:23:19.591: New orientation detected: LandscapeLeft
19:23:21.249: New orientation detected: PortraitUpsideDown
19:23:25.350: New orientation detected: LandscapeLeft
19:23:27.392: New orientation detected: FaceDown
19:23:30.101: New orientation detected: FaceUp
19:23:31.999: New orientation detected: LandscapeRight
19:23:41.704: Entering low power mode
```
Here you can see that the console will echo the current orientation state, you should see the led screen matrix match this orientation as demoed below. After holding the device still for 10 seconds, you'll see it enter low power mode.  

![Microbit Demo](docs/demo-video/proj-demo-gif.gif)

### Debugging and Testing
To debug and test the features mentioned above, I've defined two feature toggles in my `Cargo.toml` file. 

```toml
[features]
filter-debug = [] 
power-debug = []

```
1. `filter-debug`: This cargo feature defines a method that will print both the raw and filter acceleration data. This feature conveniently prints to the console and is in CVS format. I used this data to import into an excel sheet to determine the appropriate alpha value of the low pass filter (i.e $y_n = ax_n+(1-a)x_{n-1}$). Through iterative simulation of shaking the device along the x-axis, I found that an alpha value of 0.05 was best at attenuating high-frequency noise while maintaining responsiveness to intentional orientation changes. 
```
# To run this feature
cargo embed --feature "filter-debug"
```

2. `power-debug`: This feature allowed me to debug my power module and sequentially help to determine the appropriate threshold between activity and inactivity on the device based on vibrations. If vibrations of the device exceeded 75 mg, I found this to be a good indicator that the device is being held. Whereas below 75mg was about the threhold of vibration with the device sitting statically on a surface. 

```
# To run this feature
cargo embed --feature "power-debug"
```
**Note**: I don't recommend running both features together as this will flood the output console. 

### Timer Implementation Details
The application uses the Microbit's internal timer with the following specifications:
- Timer frequency: 1 MHz (1,000,000 ticks per second) -> [Source](https://docs.rs/microbit-v2/0.15.1/microbit/hal/timer/struct.Timer.html)
- Maximum 32-bit timer value: 4,294,967,295 ticks
- Maximum duration: ~71.5 minutes (4,294.967295 seconds)

This timer is used for managing the display refresh rate and power state transitions. 

**Important Note**: The current implementation does not handle timer overflow events when the 32-bit counter reaches its maximum value (~71.5 minutes). Future improvements should implement proper overflow handling to prevent potential timing issues.


### References
- [Low Pass Filter Implementation](https://dobrian.github.io/cmp/topics/filters/lowpassfilter.html) - Filter design and implementation details
- [Rust Embedded Discovery Book](https://docs.rust-embedded.org/discovery/microbit/) - Core reference for Microbit development
- [Discovery Examples](https://github.com/rust-embedded/discovery/tree/master) - Reference implementations and patterns
- [Non-blocking Screen Example](https://github.com/nrf-rs/microbit/tree/main/examples/display-nonblocking) - Reference on how to use the Microbit non-blocking screen module
- Claude AI - Assisted with documentation help, rust sytnax help, and code debugging

