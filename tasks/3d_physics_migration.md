# 3D Physics Migration

## Problem
Tank shells only worked when firing from directly left of the target due to mismatch between 2D physics (XY plane) and 3D rendering/movement (XZ plane).

## Solution
Switched from Rapier2D to Rapier3D for simpler physics integration since the game already uses 3D rendering and movement.

## Changes Made

1. **Cargo.toml**
   - Changed `bevy_rapier2d = "0.23"` to `bevy_rapier3d = "0.23"`

2. **Import Updates**
   - Updated all imports from `bevy_rapier2d::prelude::*` to `bevy_rapier3d::prelude::*`
   - Files affected: main.rs, collision.rs, projectile.rs, mech_assembly.rs

3. **Physics Configuration**
   - Changed gravity from `Vec2::ZERO` to `Vec3::ZERO`
   - Removed `pixels_per_meter` configuration (not needed for 3D)

4. **Colliders**
   - Updated enemy collider from 2D `Collider::cuboid(0.75, 0.75)` to 3D `Collider::cuboid(0.75, 0.75, 0.75)`
   - Fixed mech assembly collider to use 3D dimensions

5. **Projectile Physics**
   - Tank shells now spawn at proper Y height (0.75) matching enemy position
   - Velocity converted from Vec2 to Vec3: `Vec3::new(shell_velocity.x, 0.0, shell_velocity.y)`
   - Added `ActiveEvents::COLLISION_EVENTS` for proper collision detection

6. **Enemy Positioning**
   - Moved enemy to Y=0.75 to match visual height with physics collider

## Result
Tank shells should now work correctly from all angles, not just from the left, as the physics system now properly handles 3D collisions in the same coordinate system as the rendering.