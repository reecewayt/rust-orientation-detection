# Orientation Detection Application



todo!(), a full readme is underway. For now, please see `proposal.pdf`. 

```bash
# building 
cargo build 
# or 
cargo build --target thumbv7em-none-eabihf
```
Note that the first works because I've already specificed the target in [config.toml](.cargo/config.toml).

```bash
# flashing
cargo embed --target thumbv7em-none-eabihf

```


### This is what happens when you shake the device without filtering
```bash
00:02:41.208: New orientation: FaceUp            
00:02:41.231: New orientation: PortraitUpsideDown
00:02:41.266: New orientation: Portrait          
00:02:41.352: New orientation: PortraitUpsideDown
00:02:41.409: New orientation: Portrait          
00:02:41.501: New orientation: PortraitUpsideDown
00:02:41.522: New orientation: Portrait          
00:02:41.605: New orientation: FaceUp            
00:02:41.623: New orientation: PortraitUpsideDown
00:02:41.678: New orientation: Portrait          
00:02:41.764: New orientation: FaceUp            
00:02:41.782: New orientation: LandscapeRight    
00:02:41.817: New orientation: Portrait          
00:02:41.944: New orientation: LandscapeRight    
00:02:41.987: New orientation: Portrait          
00:02:42.074: New orientation: LandscapeRight    
00:02:42.252: New orientation: Portrait          
00:02:42.390: New orientation: LandscapeRight    
```
- TODO next is to implement filtering so that we will only record DC components of the Accelerometer data.
- TODO next implement way of tracking inactivity


# Mention the use of older dependencies and need to update but you kept them to stay compatible with the exampels found in the discovery book

To run features
```bash
cargo embed --features "power-debug"
```


### Timer Documentation
We know the timer frequency is 1 MHz (1,000,000 ticks per second) from TICKS_PER_SECOND
Maximum value for a 32-bit timer is 0xFFFFFFFF = 4,294,967,295 ticks
To get seconds: 4,294,967,295 / 1,000,000 = 4,294.967295 seconds
To get minutes: 4,294.967295 / 60 = 71.58278825 minutes