# 🐛 Mouse Look Debug Guide

## Issue Description
The camera movement (WASD) works fine, but mouse look is broken - camera gets stuck looking at the ground.

## Fixes Applied

### 1. **Controller Initialization Fix**
- **Problem**: CameraController was initialized with yaw=0, pitch=0, but camera had existing rotation
- **Fix**: Extract current camera rotation and initialize controller with those values

### 2. **Increased Mouse Sensitivity**
- **Old**: 0.002
- **New**: 0.005 (2.5x more sensitive)

### 3. **Added Debug Output**
- Mouse motion delta values
- Yaw/Pitch rotation values
- Camera position info (press F1)

## Testing Instructions

1. **Build and run the game**:
   ```bash
   cd /home/des/Rust_Projects/Chimera/Echo
   cargo run
   ```

2. **Test mouse movement**:
   - Move mouse around
   - Check console output for debug messages
   - Press F1 to see camera info

3. **Expected debug output**:
   ```
   Mouse delta: 2.34, -1.56
   Yaw: 15.23°, Pitch: -8.45°
   ```

## Debugging Steps

### If mouse still doesn't work:

1. **Check if mouse events are being received**:
   - Look for "Mouse delta" messages in console
   - If no messages appear, mouse events aren't reaching the system

2. **Check cursor lock status**:
   - Press Escape to toggle cursor lock
   - Try moving mouse when cursor is visible vs locked

3. **Check rotation values**:
   - Press F1 to see current camera rotation
   - Values should change when moving mouse

### Common Issues:

1. **No mouse events**: Cursor might not be properly locked
2. **Events but no rotation**: Rotation calculation issue
3. **Rotation but wrong direction**: Euler angle order issue

## Controls for Testing

- **WASD**: Move around (should work)
- **Mouse**: Look around (testing this)
- **Escape**: Toggle cursor lock/unlock
- **F1**: Print camera debug info
- **Space/Shift**: Up/Down movement
- **Ctrl**: Sprint

## Expected Behavior

- Smooth mouse look in all directions
- Pitch clamped to prevent camera flipping
- No getting stuck looking at ground
- Responsive mouse movement

If the issue persists, the debug output will help identify where the problem is occurring.