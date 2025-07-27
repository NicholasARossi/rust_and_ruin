# Rust and Ruin - Development Tasks

## 🎯 MVP Goal
A single Hero Mech smoothly moves to a clicked position, able to circle around a stationary Enemy Target. Hero Mech fires realistic physics-based projectiles toward the enemy on right-click.

## 📋 Development Tasks

### ✅ Step 1: Basic Bevy Setup
- [ ] Initialize Bevy App with basic structure
- [ ] Set up 2D orthographic camera
- [ ] Create game window with proper title
- [ ] Add basic background color
- [ ] Test that window opens and runs

### ✅ Step 2: Spawn Hero Mech & Enemy Target
- [ ] Create Hero entity with sprite
- [ ] Create Enemy entity with sprite
- [ ] Position Hero at center-left of screen
- [ ] Position Enemy at center-right of screen
- [ ] Add visual distinction (different colors/shapes)
- [ ] Verify both entities render correctly

### ✅ Step 3: Click-to-Move System
- [ ] Implement mouse position tracking in world coordinates
- [ ] Add left-click detection
- [ ] Create movement component for Hero
- [ ] Implement smooth movement to clicked position
- [ ] Add arrival detection and stop at target
- [ ] Implement basic circling behavior when clicking near enemy
- [ ] Test movement in all directions

### ✅ Step 4: Projectile System
- [ ] Set up bevy_rapier2d physics world
- [ ] Create projectile spawning on right-click
- [ ] Calculate aim direction from Hero to Enemy
- [ ] Apply initial velocity to projectile
- [ ] Configure gravity for realistic arc
- [ ] Add projectile sprite/visual
- [ ] Implement projectile cleanup when off-screen
- [ ] Test various shooting angles

### ✅ Step 5: Collision Detection
- [ ] Add collision shapes to Enemy and Projectiles
- [ ] Implement collision event handling
- [ ] Despawn projectile on collision
- [ ] Add visual feedback for hits
- [ ] Implement basic damage system
- [ ] Add enemy health tracking
- [ ] Log collision events for debugging
- [ ] Test collision from various angles

## 🔧 Technical Tasks

### Setup & Configuration
- [x] Initialize Rust project
- [x] Configure Cargo.toml with dependencies
- [x] Create source file structure
- [ ] Set up development environment
- [ ] Configure IDE for Rust/Bevy development

### Code Organization
- [x] Create component definitions
- [x] Create system modules
- [x] Set up resource structures
- [ ] Implement plugin architecture
- [ ] Add proper error handling

### Testing & Quality
- [ ] Add unit tests for components
- [ ] Add integration tests for systems
- [ ] Set up continuous integration
- [ ] Add performance profiling
- [ ] Document public APIs

## 🚀 Future Enhancements (Post-MVP)

### Gameplay Features
- [ ] Multiple enemy types
- [ ] Different projectile types
- [ ] Hero abilities/skills
- [ ] Enemy AI movement
- [ ] Power-ups and upgrades
- [ ] Score system
- [ ] Wave-based gameplay

### Visual Polish
- [ ] Sprite animations
- [ ] Particle effects for impacts
- [ ] UI for health/score
- [ ] Better visual assets
- [ ] Screen shake on impacts
- [ ] Projectile trails

### Audio
- [ ] Movement sounds
- [ ] Shooting sounds
- [ ] Impact sounds
- [ ] Background music
- [ ] UI feedback sounds

### Technical Improvements
- [ ] Save/load system
- [ ] Settings menu
- [ ] Performance optimizations
- [ ] Network multiplayer
- [ ] Mod support

## 📝 Notes

- Keep each step simple and functional
- Test frequently during development
- Commit working code after each completed step
- Document any issues or blockers
- Prioritize gameplay feel over visual polish for MVP

## 🐛 Known Issues

- None yet

## 💡 Ideas & Experiments

- Experiment with different physics parameters for projectiles
- Try different movement interpolation methods
- Consider adding prediction lines for projectile paths
- Test various input schemes for better game feel

## 📝 Review of Changes (2025-07-27)

### Changes made to implement rocket-style projectiles:

1. **Updated CLAUDE.md** - Clarified that this is a top-down 2D RTS game
2. **Fixed gravity** - Changed gravity from Vec2::new(0.0, -500.0) to Vec2::ZERO in main.rs for proper top-down gameplay
3. **Added Rocket component** - New component in components.rs with:
   - initial_speed: Starting speed of rocket (50.0)
   - max_speed: Maximum speed (800.0)
   - acceleration_rate: Exponential growth rate (2.5)
   - current_speed: Tracks current velocity
   - direction: Normalized direction vector
4. **Updated projectile spawning** - Modified spawn_projectile_system to:
   - Remove GravityScale component
   - Add Rocket component
   - Set initial velocity to 50.0 (slow start)
   - Changed color to orange for rockets
5. **Created rocket acceleration system** - New system that:
   - Exponentially increases speed each frame
   - Caps speed at max_speed
   - Updates velocity based on current speed
6. **Added system to update loop** - rocket_acceleration_system added to main.rs

### Result:
Projectiles now behave as rockets that start very slowly (50 units/sec) and accelerate exponentially to a maximum speed of 800 units/sec, with no gravity affecting them in the top-down view.