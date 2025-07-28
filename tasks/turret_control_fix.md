# Turret Control Fix Summary

## Problem
- Turrets were following mouse at game start
- After attack order, turrets pointed 90 degrees off from target
- Tank bodies didn't face movement direction

## Root Cause
Coordinate system mismatch between 2D mathematical conventions (0° = right/+X) and Bevy's 3D conventions (forward = +Z).

## Solution
Changed angle calculations from `atan2(y, x)` to `atan2(x, y)` to align with Bevy's coordinate system.

## Changes Made

### 1. turret_control.rs
- Fixed `calculate_turret_angle()` to use correct atan2 order
- Improved turret behavior to maintain angle when target is lost

### 2. movement.rs
- Fixed tank rotation to use correct atan2 order

### 3. mech_tests.rs
- Updated test expectations for new coordinate system

## Result
- Turrets correctly aim at enemies
- Tanks face their movement direction
- Consistent coordinate system throughout