# Tank Shell Physics Update

## Changes Made

### 1. Tank Shell Speed & Fire Rate
- ✅ Increased TANK_SHELL_SPEED from 8.0 to 15.0 for faster, more impactful shells
- ✅ Increased fire_rate in TurretCannon from 0.2 to 1.5 seconds for slower, more deliberate shots

### 2. Physics System Overhaul
- ✅ Changed tank shells from KinematicPositionBased to Dynamic rigid bodies
- ✅ Added physical properties to shells:
  - High density (10.0) for heavy shells
  - Restitution coefficient (0.4) for slight bouncing
  - Friction coefficient (0.3)
  - Continuous collision detection (CCD) for fast projectiles
  - Slight gravity scale (0.3) for realistic arc

### 3. Enemy Physics Response
- ✅ Changed enemies from Fixed to Dynamic rigid bodies
- ✅ Added mass properties (density 5.0) to make enemies heavy
  - Locked rotation and Y-axis movement to keep enemies upright
  - Added damping to quickly stop movement after impact

### 4. Impact System
- ✅ Enhanced collision system to apply impulse forces on shell impact
- ✅ Added knockback effect with 50.0 force multiplier
- ✅ Tank shells now ricochet off enemies instead of despawning immediately

### 5. Visual Feedback
- ✅ Added HitFlash component for visual impact feedback
- ✅ Created hit_flash_system that flashes enemies white when hit
- ✅ Flash effect lasts 0.2 seconds and fades smoothly

## Review

The tank shell system now feels much more impactful with:
- Faster projectiles that arc slightly due to gravity
- Proper physics interactions including ricochets
- Visual knockback when enemies are hit
- Flash effect to emphasize impacts
- Slower fire rate that makes each shot feel more significant

The game now has a more visceral, physics-based combat feel where shells have real weight and momentum.