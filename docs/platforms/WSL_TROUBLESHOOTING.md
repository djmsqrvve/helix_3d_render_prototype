# WSL Window Display Troubleshooting Guide

> **Note:** This project has transitioned to **Pure Ubuntu** as the primary development environment. This guide is maintained for students and contributors still using WSL2.

## Issue Summary

Window appears in taskbar but not visible on screen, or window never appears despite process running.

## Root Causes Identified

### 1. WSLg Shared Memory Failure (Primary Issue)

**Error Pattern:**
```
rdp_allocate_shared_memory: Failed to open "/mnt/shared_memory/{...}" with error: Input/output error
```

**Check:**
```bash
ls /mnt/shared_memory/
# If "No such file or directory" - WSLg shared memory is broken
```

**Cause:** WSLg's shared memory filesystem is not mounted or has crashed, usually due to:
- Hung GPU processes
- Memory pressure
- WSLg service crash

**Solution:**
```bash
# Windows PowerShell (Admin)
wsl --shutdown

# Or full restart of WSLg:
wsl --terminate Ubuntu
wsl
```

### 2. Wayland vs X11 Backend

**Problem:** Bevy defaults to Wayland if `WAYLAND_DISPLAY` is set, but WSLg's Wayland doesn't always work properly.

**Environment When Working:**
```bash
export DISPLAY=:0
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY
```

**Verify Backend:**
```bash
echo "DISPLAY=$DISPLAY"
echo "WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
echo "WINIT_UNIX_BACKEND=$WINIT_UNIX_BACKEND"
```

### 3. Hung Processes

**Check for zombie/hung processes:**
```bash
ps aux | grep -E "bevy|cargo|rustc" | grep -v grep
```

**Kill all:**
```bash
pkill -9 -f "bevy-3d-renderer"
pkill -9 -f "cargo"
pkill -9 -f "rustc"
```

### 4. Build Locks

**Check for stuck locks:**
```bash
ls target/gltf_test/.cargo-lock
# If exists, remove:
rm target/gltf_test/.cargo-lock
```

## Diagnostic Commands

### Check WSLg Health
```bash
# Check WSLg version
cat /mnt/wslg/versions.txt

# Check shared memory
ls -la /mnt/shared_memory/ 2>&1

# Check weston logs for errors
tail -20 /mnt/wslg/weston.log | grep -i error

# Check X11 socket
ls -la /tmp/.X11-unix/X0

# Check display environment
echo "DISPLAY=$DISPLAY"
echo "WAYLAND_DISPLAY=$WAYLAND_DISPLAY"
```

### Check System Resources
```bash
# Memory
free -h

# Disk space
df -h

# Zombie processes
ps aux | grep "<defunct>"
```

## Working Run Configuration

```bash
cd ~/helix_3d_render_prototype

# Force X11 backend
export DISPLAY=:0
export WINIT_UNIX_BACKEND=x11
unset WAYLAND_DISPLAY

# Kill any stuck processes
pkill -9 -f "bevy|cargo|rustc" 2>/dev/null
sleep 1

# Clear locks
rm -f target/gltf_test/.cargo-lock 2>/dev/null

# Run
cargo run
```

## If Window Still Doesn't Appear

1. **Check taskbar:** Look for "Drow Ranger - Complete Assembly" icon
2. **Alt+Tab:** Window may be minimized
3. **Windows Key + Arrow:** Try maximizing with Windows+Up
4. **Check logs:** Look for "Creating new window" message

## Last Resort Solutions

### 1. Full WSL Restart
```powershell
# Windows PowerShell (Admin)
wsl --shutdown
# Wait 10 seconds, then reopen Ubuntu
```

### 2. Windows Restart
Sometimes WSLg requires full Windows restart to reset graphics subsystem.

### 3. Alternative: Run on Native Windows
```powershell
# On Windows (not WSL)
cd C:\path\to\helix_3d_render_prototype
cargo run
```

## What We Learned

1. **Code is fine** - Window configuration hasn't changed from working version
2. **WSLg can break** - Shared memory filesystem can crash
3. **X11 works better** - Force `WINIT_UNIX_BACKEND=x11` for WSL2
4. **Process cleanup matters** - Hung processes can hold resources

## Related Documentation

- `DEV_TROUBLESHOOTING.md` - General runtime issues
- `animation_indices.md` - Animation system
- `docs/wsl_window_debug_session_2026-02-15.md` - This debugging session

## When to Suspect Code Issues

If after WSL restart and clean environment the window still doesn't appear, check:
- Did window title change? (regression test checks this)
- Did window position settings change?
- Run `make regression-test` to verify setup

## Status

**Date:** 2026-02-15  
**Issue:** WSLg shared memory failure  
**Workaround:** Force X11 backend + WSL restart  
**Permanent Fix:** None yet - appears to be WSL infrastructure issue
