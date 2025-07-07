# 🎮 Controls Test Guide

## Updated Controls

### Movement (QWSD Layout)
- **W**: Forward
- **S**: Backward  
- **Q**: Left (strafe)
- **D**: Right (strafe)
- **Space**: Up
- **Left Shift**: Down
- **Left Ctrl**: Sprint

### Camera
- **Mouse**: Look around
- **Escape**: Toggle cursor lock/unlock
- **F1**: Debug camera info

## Fixes Applied

### 1. **Key Mapping Fixed**
- Changed from WASD to QWSD layout
- Q = left strafe, D = right strafe

### 2. **Mouse Look Completely Rebuilt**
- Simplified rotation system
- Using quaternion multiplication instead of Euler angles
- Reset camera to neutral position on startup
- Improved pitch clamping

### 3. **Camera Initialization**
- Camera starts at (0, 1, 0) with no rotation
- Controller starts with yaw=0, pitch=0
- Clean slate for better control

## Testing Steps

1. **Build and run**:
   ```bash
   cd /home/des/Rust_Projects/Chimera/Echo
   cargo build
   cargo run
   ```

2. **Test movement**:
   - **Q**: Should move left
   - **W**: Should move forward
   - **S**: Should move backward
   - **D**: Should move right

3. **Test mouse look**:
   - Move mouse left/right: Should turn camera horizontally
   - Move mouse up/down: Should look up/down
   - Should NOT get stuck looking at ground

4. **Test cursor**:
   - Press Escape to unlock cursor
   - Move mouse - cursor should be visible and movable
   - Press Escape again to lock cursor
   - Mouse should control camera again

## Expected Behavior

- **Smooth mouse look** in all directions
- **No getting stuck** looking at ground
- **Responsive movement** with QWSD keys
- **Proper cursor locking/unlocking**

## If Issues Persist

1. **Check console output** for any error messages
2. **Press F1** to see camera position/rotation
3. **Try Escape** to toggle cursor lock
4. **Test with cursor unlocked** vs locked

The mouse look system has been completely rebuilt to be more reliable!