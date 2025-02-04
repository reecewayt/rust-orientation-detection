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
