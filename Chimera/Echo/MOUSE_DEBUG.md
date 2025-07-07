# 🐛 Mouse Camera Debug Guide

## Debug Information Added

I've added debug output to the mouse look system to help identify the issue:

1. **Mouse motion detection**: Shows if mouse events are being received
2. **Rotation values**: Shows yaw/pitch calculations  
3. **Final rotation**: Shows the quaternion being applied to camera

## Testing Steps

1. **Build and run**:
   ```bash
   cd /home/des/Rust_Projects/Chimera/Echo
   cargo build
   cargo run
   ```

2. **Test mouse movement**:
   - Move your mouse around
   - Watch the console output
   - Look for debug messages

## Expected Debug Output

When you move the mouse, you should see:
```
Mouse motion: x=2.34, y=-1.56
Yaw: 15.23°, Pitch: -8.45°
Final rotation: Quat(0.123, 0.456, 0.789, 0.321)
```

## Diagnosis

### If you see NO debug output:
- Mouse events are not being captured
- Cursor might not be locked properly
- Try pressing Escape to toggle cursor lock

### If you see mouse motion but no rotation change:
- Sensitivity might be too low
- Calculation error in yaw/pitch

### If you see rotation values but camera doesn't move:
- Problem with applying rotation to transform
- Issue with Euler angle conversion

### If camera moves but gets stuck:
- Pitch clamping issue
- Euler angle gimbal lock

## Quick Tests

1. **Press F1** to see current camera info
2. **Press Escape** to toggle cursor lock/unlock
3. **Try moving mouse when cursor is visible vs locked**

## Controls (unchanged)
- **WASD**: Movement (working fine)
- **Mouse**: Look around (debugging this)
- **Escape**: Toggle cursor lock
- **F1**: Camera debug info

The debug output will help us identify exactly where the mouse look system is failing.