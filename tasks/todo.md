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