use bevy::prelude::*;
use crate::mech::*;
use crate::rendering::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Team {
    Player,
    Enemy,
}

pub struct MechSpawnParams {
    pub name: String,
    pub position: Vec3,
    pub rotation: f32,
    pub team: Team,
}

pub struct MechAssemblyResult {
    pub root_entity: Entity,
    pub lower_entity: Entity,
    pub upper_entity: Entity,
    pub barrel_entity: Entity,
}

impl MechAssemblyResult {
    pub fn is_valid(&self) -> bool {
        self.root_entity != self.lower_entity
            && self.root_entity != self.upper_entity
            && self.root_entity != self.barrel_entity
            && self.lower_entity != self.upper_entity
            && self.lower_entity != self.barrel_entity
            && self.upper_entity != self.barrel_entity
    }
}

pub fn spawn_mech(
    commands: &mut Commands,
    name: &str,
    tank_treads: TankTreads,
    turret_cannon: TurretCannon,
    transform: Transform,
) -> Entity {
    let root_entity = commands
        .spawn((
            Mech::new(name),
            MechParts::new(),
            transform,
            GlobalTransform::default(),
        ))
        .id();

    let lower_entity = commands
        .spawn((
            MechLowerPart,
            MechLower {
                turn_rate: tank_treads.turn_rate,
                max_speed: tank_treads.speed,
            },
            tank_treads.clone(),
            TransformBundle {
                local: Transform::from_translation(get_mech_part_offset(MechPartType::TankTreads)),
                ..default()
            },
        ))
        .id();

    let upper_entity = commands
        .spawn((
            MechUpperPart,
            MechUpper {
                rotation_speed: turret_cannon.rotation_speed,
                weapon_mount_offset: Vec3::new(0.0, 0.0, turret_cannon.barrel_length / 2.0),
            },
            turret_cannon.clone(),
            TurretRotation {
                target_angle: 0.0,
                current_angle: 0.0,
            },
            TransformBundle {
                local: Transform::from_translation(get_mech_part_offset(MechPartType::TurretBase)),
                ..default()
            },
        ))
        .id();

    let barrel_entity = commands
        .spawn((
            CannonBarrel,
            TransformBundle {
                local: Transform::from_translation(get_mech_part_offset(MechPartType::CannonBarrel)),
                ..default()
            },
        ))
        .id();

    commands.entity(root_entity).push_children(&[lower_entity, upper_entity]);
    commands.entity(upper_entity).push_children(&[barrel_entity]);

    commands.entity(root_entity).insert(MechParts {
        lower: Some(lower_entity),
        upper: Some(upper_entity),
    });

    root_entity
}

pub fn spawn_mech_with_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    name: &str,
    tank_treads: TankTreads,
    turret_cannon: TurretCannon,
    transform: Transform,
) -> MechAssemblyResult {
    let root_entity = commands
        .spawn((
            Mech::new(name),
            MechParts::new(),
            SpatialBundle {
                transform,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::cuboid(0.3, 0.4),
        ))
        .id();

    let lower_mesh = meshes.add(create_tank_tread_mesh(Vec2::new(1.2, 1.6)));
    let lower_material = create_mech_material(MechPartType::TankTreads, materials);

    let lower_entity = commands
        .spawn((
            MechLowerPart,
            MechLower {
                turn_rate: tank_treads.turn_rate,
                max_speed: tank_treads.speed,
            },
            tank_treads,
            PbrBundle {
                mesh: lower_mesh,
                material: lower_material,
                transform: Transform::from_translation(get_mech_part_offset(MechPartType::TankTreads)),
                ..default()
            },
        ))
        .id();

    let upper_mesh = meshes.add(create_turret_base_mesh(0.4));
    let upper_material = create_mech_material(MechPartType::TurretBase, materials);

    let upper_entity = commands
        .spawn((
            MechUpperPart,
            MechUpper {
                rotation_speed: turret_cannon.rotation_speed,
                weapon_mount_offset: Vec3::new(0.0, 0.0, turret_cannon.barrel_length / 2.0),
            },
            turret_cannon.clone(),
            TurretRotation {
                target_angle: 0.0,
                current_angle: 0.0,
            },
            PbrBundle {
                mesh: upper_mesh,
                material: upper_material,
                transform: Transform::from_translation(get_mech_part_offset(MechPartType::TurretBase)),
                ..default()
            },
        ))
        .id();

    let barrel_mesh = meshes.add(create_cannon_barrel_mesh(turret_cannon.barrel_length * 2.0, 0.2));
    let barrel_material = create_mech_material(MechPartType::CannonBarrel, materials);

    let barrel_entity = commands
        .spawn((
            CannonBarrel,
            PbrBundle {
                mesh: barrel_mesh,
                material: barrel_material,
                transform: Transform::from_translation(get_mech_part_offset(MechPartType::CannonBarrel)),
                ..default()
            },
        ))
        .id();

    commands.entity(root_entity).push_children(&[lower_entity, upper_entity]);
    commands.entity(upper_entity).push_children(&[barrel_entity]);

    commands.entity(root_entity).insert(MechParts {
        lower: Some(lower_entity),
        upper: Some(upper_entity),
    });

    MechAssemblyResult {
        root_entity,
        lower_entity,
        upper_entity,
        barrel_entity,
    }
}

pub fn get_barrel_tip_position(turret_transform: &Transform, barrel_length: f32) -> Vec3 {
    let forward = turret_transform.rotation * Vec3::Z;
    turret_transform.translation + forward * barrel_length
}

pub fn calculate_mech_transform(position: Vec3, rotation: f32) -> Transform {
    Transform::from_translation(position)
        .with_rotation(Quat::from_rotation_y(rotation.to_radians()))
}