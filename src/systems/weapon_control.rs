use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Hero, Enemy, Projectile, TankShell, AttackTarget};
use crate::mech::{MechWeapon, MechUpperBody, MechRotation, CannonWeapon};
use crate::systems::upper_body_control::is_upper_facing_target;

const ATTACK_RANGE: f32 = 10.0;
const ANGLE_TOLERANCE: f32 = 5.0;

pub fn weapon_control_system(
    mut commands: Commands,
    time: Res<Time>,
    hero_query: Query<(&Transform, &Children, &AttackTarget), With<Hero>>,
    upper_query: Query<(&Transform, &GlobalTransform, &MechUpperBody, &MechRotation, &Children)>,
    mut weapon_query: Query<(&mut MechWeapon, &CannonWeapon, &Parent)>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let delta_time = time.delta_seconds();
    
    for (hero_transform, children, attack_target) in hero_query.iter() {
        if let Ok(enemy_transform) = enemy_query.get(attack_target.entity) {
            let hero_pos = Vec2::new(hero_transform.translation.x, hero_transform.translation.z);
            let enemy_pos = Vec2::new(enemy_transform.translation.x, enemy_transform.translation.z);
            let distance = hero_pos.distance(enemy_pos);
            
            if distance <= ATTACK_RANGE {
                for child in children {
                    if let Ok((upper_transform, global_upper_transform, _upper_body, _rotation, upper_children)) = upper_query.get(*child) {
                        let upper_position = Vec2::new(global_upper_transform.translation().x, global_upper_transform.translation().z);
                        let is_facing = is_upper_facing_target(upper_transform, upper_position, enemy_pos, ANGLE_TOLERANCE);
                        
                        if is_facing {
                            for upper_child in upper_children {
                                if let Ok((mut weapon, cannon, _parent)) = weapon_query.get_mut(*upper_child) {
                                    weapon.last_fire_time += delta_time;
                                    
                                    if weapon.last_fire_time >= weapon.weapon_stats.fire_rate {
                                        fire_cannon(
                                            &mut commands,
                                            &global_upper_transform,
                                            upper_transform,
                                            weapon.as_ref(),
                                            cannon,
                                            enemy_pos,
                                            &mut meshes,
                                            &mut materials,
                                        );
                                        
                                        weapon.last_fire_time = 0.0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn fire_cannon(
    commands: &mut Commands,
    global_upper_transform: &GlobalTransform,
    local_upper_transform: &Transform,
    weapon: &MechWeapon,
    cannon: &CannonWeapon,
    enemy_pos: Vec2,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let barrel_tip = get_barrel_tip_position(global_upper_transform, local_upper_transform, weapon, cannon.barrel_length);
    let spawn_pos = Vec2::new(barrel_tip.x, barrel_tip.z);
    
    let direction = (enemy_pos - spawn_pos).normalize();
    let shell_velocity = direction * weapon.weapon_stats.projectile_speed;
    
    let shell_mesh = meshes.add(Mesh::from(shape::Box::new(0.4, 0.2, 0.4)));
    let shell_material = materials.add(Color::rgb(1.0, 1.0, 0.0).into());
    
    commands.spawn((
        Projectile {
            damage: weapon.weapon_stats.damage,
            speed: weapon.weapon_stats.projectile_speed,
        },
        TankShell {
            velocity: shell_velocity,
            spawn_position: spawn_pos,
            max_range: weapon.weapon_stats.range,
        },
        PbrBundle {
            mesh: shell_mesh,
            material: shell_material,
            transform: Transform::from_xyz(spawn_pos.x, 0.75, spawn_pos.y),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(0.2),
        ColliderMassProperties::Density(10.0),
        Restitution::coefficient(0.4),
        Friction::coefficient(0.3),
        Ccd::enabled(),
        Velocity {
            linvel: Vec3::new(shell_velocity.x, 0.0, shell_velocity.y),
            angvel: Vec3::ZERO,
        },
        ExternalImpulse::default(),
        GravityScale(0.3),
        ActiveEvents::COLLISION_EVENTS
    ));
    
    info!("Cannon fired from hardpoint {} at pos ({}, {}, {}) with velocity {:?}", 
          weapon.hardpoint_id, spawn_pos.x, 0.75, spawn_pos.y, shell_velocity);
}

fn get_barrel_tip_position(
    global_transform: &GlobalTransform,
    local_transform: &Transform,
    weapon: &MechWeapon,
    barrel_length: f32,
) -> Vec3 {
    let hardpoint = &weapon.hardpoint_id;
    let offset = if hardpoint == "left" {
        Vec3::new(-0.3, 0.0, barrel_length)
    } else if hardpoint == "right" {
        Vec3::new(0.3, 0.0, barrel_length)
    } else {
        Vec3::new(0.0, 0.0, barrel_length)
    };
    
    global_transform.translation() + global_transform.to_scale_rotation_translation().1 * offset
}