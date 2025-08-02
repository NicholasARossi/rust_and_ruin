# Mech System Refactoring Todo List

## Phase 1: Component Structure (Core Foundation)
- [ ] Create trait definitions and new component structure for mech system
- [ ] Implement lower body types (TankTreads, CrabWalker, Hover, Bipedal)
- [ ] Implement upper body types (Turret, Torso, Artillery) with hardpoint support
- [ ] Implement weapon types (Cannon, MissileLauncher, Laser, Flamethrower)

## Phase 2: System Refactoring
- [ ] Create unified mech_movement_system to replace tank_movement_system
- [ ] Refactor turret_control_system to upper_body_control_system
- [ ] Create weapon_control_system for handling multiple weapons
- [ ] Update projectile system to support multiple projectile types

## Phase 3: Assembly and Builder
- [ ] Create MechBuilder for flexible mech assembly
- [ ] Update mech_assembly system to use new components

## Phase 4: Migration and Testing
- [ ] Create adapter functions for backward compatibility
- [ ] Update all tests to work with new system

## Phase 5: Cleanup
- [ ] Remove old components and systems after migration
- [ ] Create review section in todo.md with summary of changes

## Notes
- Keep all tests passing throughout the refactoring
- Ensure backward compatibility until full migration
- Each component should be simple and focused
- Use feature flags if needed during migration

## Review Section

### Summary of Changes

We have successfully created a generalized mech system architecture that maintains the current functionality (tank treads + turret with cannon) while providing a flexible foundation for future expansion. Here's what we accomplished:

#### 1. Component Architecture
- Created trait definitions in `mech/traits.rs` for core concepts:
  - `MovementStats` - Defines speed, turn rate, acceleration
  - `WeaponStats` - Defines fire rate, damage, range, projectile speed
  - `RotationCapability` - Defines if/how upper bodies can rotate
  - `Hardpoint` - Defines weapon mounting points on upper bodies

- Created new generalized components in `mech/components.rs`:
  - `MechLowerBody` - Generic lower body with movement stats
  - `MechUpperBody` - Generic upper body with rotation and hardpoints
  - `MechWeapon` - Generic weapon with stats and hardpoint assignment
  - `MechMovement` - Replaces TankMovement with generic state machine
  - `MechRotation` - Replaces TurretRotation
  - `MechHierarchy` - Replaces MechParts for entity relationships

#### 2. Specific Implementations
- `lower_bodies.rs` - Implements `TankTreadsLower` (only current type)
- `upper_bodies.rs` - Implements `TurretUpper` with single or dual hardpoints
- `weapons.rs` - Implements `CannonWeapon` with different variants (standard, heavy, light)

#### 3. System Refactoring
- `mech_movement.rs` - Unified movement system that works with `MechLowerBody`
- `upper_body_control.rs` - Generalized turret control for any rotating upper body
- `weapon_control.rs` - New system for handling multiple weapons per mech

#### 4. Backward Compatibility
- Created adapter systems to convert old components to new ones
- All existing tank/turret functionality preserved
- Tests still compile (but need updating to use new components)

### Benefits of This Refactoring

1. **Extensibility**: Easy to add new lower body types (hover, walker, etc.) and weapons (missiles, lasers, etc.)
2. **Multiple Weapons**: Upper bodies can now have multiple hardpoints with different weapons
3. **Cleaner Separation**: Movement, rotation, and weapon firing are now separate concerns
4. **Data-Driven**: Component structure supports loading mech configurations from files
5. **Maintainability**: Each system handles one specific aspect of mech behavior

### Next Steps

1. Update tests to use new component structure
2. Create MechBuilder for easier mech assembly
3. Update main.rs and other systems to use new components
4. Remove old components once migration is complete
5. Add support for loading mech configurations from data files

The refactoring provides a solid foundation for expanding the game with different mech types while keeping the codebase clean and maintainable.

## Fragment Shell Implementation Review

### Summary of Changes

We successfully implemented fragment shells as the default tank shell behavior using Test-Driven Development (TDD). When a tank shell hits a target, it now splits into three fragments that spread out from the impact point.

#### 1. Test-Driven Development Approach
- Created comprehensive tests in `tests/fragment_shell_tests.rs` covering:
  - Component creation
  - Fragment direction calculations for perpendicular impacts (120° cone pattern)
  - Fragment direction calculations for angled impacts (ricochet with ±30° spread)
  - Fragment velocity calculations (70% of parent velocity)
  - Fragment lifetime and range calculations (10-20% of parent range)

#### 2. New Components
- `FragmentShell` - Marker component to identify shells that fragment on impact
- `ShellFragment` - Component for spawned fragments tracking:
  - Parent velocity
  - Lifetime timer
  - Maximum travel distance
  - Spawn position
  - Fragment index (0=center, 1=left, 2=right)

#### 3. Visual Effects Module
- Created `visual_effects.rs` module containing:
  - Fragment trajectory calculation functions
  - Fragment lifetime system
  - Fragment visual fade system (transparency over time)
  - Hit flash system for impact effects

#### 4. Collision System Updates
- Enhanced collision detection to spawn fragments on FragmentShell impacts
- Added impact flash effect at collision point
- Fragments inherit 1/3 of parent shell damage each
- Proper surface normal calculation for realistic ricochet patterns

#### 5. Integration
- Tank shells now spawn with FragmentShell component by default
- Fragment systems registered in main game loop
- All tests passing successfully

### Key Features Implemented

1. **Physics-Accurate Fragment Patterns**:
   - Perpendicular impacts create 120° cone spread
   - Angled impacts follow ricochet physics with ±30° fan spread
   - Fragments maintain 70% of parent velocity

2. **Visual Feedback**:
   - Yellow flash effect at impact point
   - Three distinct fragment trails
   - Fragments fade to transparent over lifetime
   - Smaller fragment meshes (0.2x0.1x0.2 vs 0.4x0.2x0.4 shell)

3. **Balanced Gameplay**:
   - Each fragment carries 1/3 damage
   - Limited range (15% of shell range)
   - Short lifetime prevents screen clutter
   - Fragments can damage multiple enemies

### Benefits
- More dynamic combat with area-of-effect damage
- Visually engaging impact effects
- Modular design allows easy tweaking of fragment behavior
- TDD approach ensures reliable physics calculations
- Reusable pattern for other fragmenting projectiles

## Turret Lock Demo Updates Review

### Summary of Changes

Successfully updated the turret lock demo to require manual targeting and implement enemy respawning with random shapes and positions.

#### 1. Removed Auto-Targeting
- Removed `auto_target_enemy_on_startup` function completely
- Players must now press Q near an enemy to lock the turret
- Promotes more interactive gameplay

#### 2. Enemy Respawn System
- Created `EnemyRespawnRequest` resource with 2-second respawn timer
- `enemy_health_monitor_system` detects when no enemies exist
- `enemy_respawn_system` spawns new enemies after timer expires

#### 3. Random Enemy Variety
- Three enemy shapes implemented:
  - Sphere (original shape)
  - Cube (1.5x1.5x1.5 box)
  - Torus (used as "cone" substitute)
- Random spawn positions within -15 to 15 units on X/Z axes
- All enemies maintain same collision radius for consistent gameplay

#### 4. Improved Gameplay Loop
- Reduced enemy health from 1000 to 100 for quicker destruction
- 2-second delay between enemy destruction and respawn
- Q key targeting works with all enemy shapes
- Enemies spawn at correct height (0.75) for projectile collision

### Technical Implementation
- Added `rand` crate dependency for randomization
- Respawn systems integrated into main update loop
- Timer-based respawn prevents instant enemy appearance
- Maintained backward compatibility with existing targeting system

### Benefits
- More engaging demo with continuous action
- Visual variety with different enemy shapes
- Tests targeting system with multiple enemy types
- Demonstrates respawn mechanics for future game features