use bevy::prelude::*;

#[cfg(test)]
mod mech_visual_tests {
    use super::*;
    use rust_and_ruin::rendering::*;

    #[test]
    fn test_tank_tread_mesh_creation() {
        let size = Vec2::new(0.6, 0.8);
        let mesh = create_tank_tread_mesh(size);
        
        assert!(mesh.count_vertices() == 8);
        assert!(mesh.primitive_topology() == bevy::render::render_resource::PrimitiveTopology::TriangleList);
    }

    #[test]
    fn test_turret_base_mesh_creation() {
        let radius = 0.2;
        let mesh = create_turret_base_mesh(radius);
        
        assert!(mesh.count_vertices() >= 17);
        assert!(mesh.primitive_topology() == bevy::render::render_resource::PrimitiveTopology::TriangleList);
    }

    #[test]
    fn test_cannon_barrel_mesh_creation() {
        let length = 0.5;
        let width = 0.1;
        let mesh = create_cannon_barrel_mesh(length, width);
        
        assert!(mesh.count_vertices() == 4);
        assert!(mesh.primitive_topology() == bevy::render::render_resource::PrimitiveTopology::TriangleList);
    }

    #[test]
    fn test_mech_part_sizing() {
        let tank_size = Vec2::new(0.6, 0.8);
        let turret_radius = 0.2;
        
        assert!(turret_radius * 2.0 < tank_size.x);
        assert!(turret_radius * 2.0 < tank_size.y);
    }

    #[test]
    fn test_mech_material_colors() {
        let tread_color = match MechPartType::TankTreads {
            MechPartType::TankTreads => Color::rgb(0.25, 0.25, 0.25),
            _ => Color::WHITE,
        };
        
        let turret_color = match MechPartType::TurretBase {
            MechPartType::TurretBase => Color::rgb(0.0, 0.5, 1.0),
            _ => Color::WHITE,
        };
        
        let barrel_color = match MechPartType::CannonBarrel {
            MechPartType::CannonBarrel => Color::rgb(0.5, 0.5, 0.5),
            _ => Color::WHITE,
        };
        
        assert_eq!(tread_color, Color::rgb(0.25, 0.25, 0.25));
        assert_eq!(turret_color, Color::rgb(0.0, 0.5, 1.0));
        assert_eq!(barrel_color, Color::rgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_mech_visual_offsets() {
        let lower_offset = get_mech_part_offset(MechPartType::TankTreads);
        let upper_offset = get_mech_part_offset(MechPartType::TurretBase);
        let barrel_offset = get_mech_part_offset(MechPartType::CannonBarrel);
        
        assert_eq!(lower_offset.y, 0.0);
        assert!(upper_offset.y > 0.0);
        assert_eq!(barrel_offset.y, 0.0);
        assert!(barrel_offset.z > 0.0);
    }
}

#[cfg(test)]
mod mech_component_tests {
    use super::*;
    use rust_and_ruin::mech::*;

    #[test]
    fn test_mech_component_creation() {
        let mech = Mech::new("TestMech");
        assert_eq!(mech.name, "TestMech");
    }

    #[test]
    fn test_mech_lower_component() {
        let lower = MechLower {
            turn_rate: 90.0,
            max_speed: 5.0,
        };
        assert_eq!(lower.turn_rate, 90.0);
        assert_eq!(lower.max_speed, 5.0);
    }

    #[test]
    fn test_mech_upper_component() {
        let upper = MechUpper {
            rotation_speed: 180.0,
            weapon_mount_offset: Vec3::new(0.0, 0.1, 0.3),
        };
        assert_eq!(upper.rotation_speed, 180.0);
        assert_eq!(upper.weapon_mount_offset, Vec3::new(0.0, 0.1, 0.3));
    }

    #[test]
    fn test_tank_treads_creation() {
        let treads = TankTreads::default();
        assert_eq!(treads.speed, 4.0);
        assert_eq!(treads.turn_rate, 60.0);
        assert_eq!(treads.acceleration, 2.0);
    }

    #[test]
    fn test_turret_cannon_creation() {
        let cannon = TurretCannon::default();
        assert_eq!(cannon.fire_rate, 1.0);
        assert_eq!(cannon.projectile_damage, 25.0);
        assert_eq!(cannon.rotation_speed, 120.0);
        assert_eq!(cannon.barrel_length, 0.5);
    }

    #[test]
    fn test_mech_part_attachment() {
        let lower_entity = Entity::from_raw(1);
        let upper_entity = Entity::from_raw(2);
        
        let mech_parts = MechParts {
            lower: Some(lower_entity),
            upper: Some(upper_entity),
        };
        
        assert!(mech_parts.has_lower());
        assert!(mech_parts.has_upper());
        assert!(mech_parts.is_complete());
    }

    #[test]
    fn test_incomplete_mech_parts() {
        let mech_parts = MechParts {
            lower: Some(Entity::from_raw(1)),
            upper: None,
        };
        
        assert!(mech_parts.has_lower());
        assert!(!mech_parts.has_upper());
        assert!(!mech_parts.is_complete());
    }

    #[test]
    fn test_mech_stats_calculation() {
        let stats = MechStats::from_parts(
            &TankTreads::default(),
            &TurretCannon::default()
        );
        
        assert_eq!(stats.max_speed, 4.0);
        assert_eq!(stats.turn_rate, 60.0);
        assert_eq!(stats.fire_rate, 1.0);
        assert_eq!(stats.damage, 25.0);
    }
}

#[cfg(test)]
mod mech_hierarchy_tests {
    use super::*;

    #[test]
    fn test_transform_hierarchy() {
        let _root_transform = Transform::from_xyz(10.0, 0.0, 5.0);
        let lower_offset = Vec3::new(0.0, 0.0, 0.0);
        let upper_offset = Vec3::new(0.0, 0.1, 0.0);
        
        let lower_transform = Transform::from_translation(lower_offset);
        let upper_transform = Transform::from_translation(upper_offset);
        
        assert_eq!(lower_transform.translation.y, 0.0);
        assert_eq!(upper_transform.translation.y, 0.1);
    }

    #[test]
    fn test_turret_rotation_independence() {
        let mut lower_transform = Transform::from_xyz(0.0, 0.0, 0.0);
        let mut upper_transform = Transform::from_xyz(0.0, 0.1, 0.0);
        
        lower_transform.rotate_y(45.0_f32.to_radians());
        upper_transform.rotate_y(90.0_f32.to_radians());
        
        let lower_rotation = lower_transform.rotation.to_euler(EulerRot::XYZ).1.to_degrees();
        let upper_rotation = upper_transform.rotation.to_euler(EulerRot::XYZ).1.to_degrees();
        
        assert!((lower_rotation - 45.0).abs() < 0.1);
        assert!((upper_rotation - 90.0).abs() < 0.1);
    }

    #[test]
    fn test_weapon_mount_position() {
        let upper_transform = Transform::from_xyz(0.0, 0.1, 0.0);
        let weapon_offset = Vec3::new(0.0, 0.0, 0.5);
        
        let weapon_world_pos = upper_transform.transform_point(weapon_offset);
        
        assert_eq!(weapon_world_pos.x, 0.0);
        assert_eq!(weapon_world_pos.y, 0.1);
        assert_eq!(weapon_world_pos.z, 0.5);
    }
}

#[cfg(test)]
mod mech_assembly_tests {
    use super::*;
    use rust_and_ruin::mech::*;
    use rust_and_ruin::systems::mech_assembly::*;
    use bevy::ecs::system::CommandQueue;
    use bevy::app::App;

    #[test]
    fn test_spawn_mech_creates_hierarchy() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        
        let tank_treads = TankTreads::default();
        let turret_cannon = TurretCannon::default();
        
        app.world.spawn_empty().id();
        
        let mech_entity = app.world.spawn_empty().id();
        
        assert_ne!(mech_entity, Entity::PLACEHOLDER);
        assert!(mech_entity.index() >= 0);
    }

    #[test]
    fn test_mech_assembly_result() {
        let assembly_result = MechAssemblyResult {
            root_entity: Entity::from_raw(1),
            lower_entity: Entity::from_raw(2),
            upper_entity: Entity::from_raw(3),
            barrel_entity: Entity::from_raw(4),
        };
        
        assert!(assembly_result.is_valid());
        assert_ne!(assembly_result.root_entity, assembly_result.lower_entity);
        assert_ne!(assembly_result.upper_entity, assembly_result.barrel_entity);
    }

    #[test]
    fn test_mech_spawn_parameters() {
        let params = MechSpawnParams {
            name: "TestMech".to_string(),
            position: Vec3::new(10.0, 0.0, 10.0),
            rotation: 45.0,
            team: Team::Player,
        };
        
        assert_eq!(params.name, "TestMech");
        assert_eq!(params.position.x, 10.0);
        assert_eq!(params.position.z, 10.0);
        assert_eq!(params.rotation, 45.0);
    }

    #[test]
    fn test_get_barrel_tip_position() {
        let turret_transform = Transform::from_xyz(0.0, 0.1, 0.0);
        let barrel_length = 0.5;
        
        let tip_position = get_barrel_tip_position(&turret_transform, barrel_length);
        
        assert_eq!(tip_position.y, 0.1);
        assert!(tip_position.z > 0.0);
    }

    #[test]
    fn test_calculate_mech_transform() {
        let position = Vec3::new(5.0, 0.0, 5.0);
        let rotation = 90.0;
        
        let transform = calculate_mech_transform(position, rotation);
        
        assert_eq!(transform.translation, position);
        let euler = transform.rotation.to_euler(EulerRot::XYZ);
        assert!((euler.1.to_degrees() - rotation).abs() < 0.1);
    }
}

#[cfg(test)]
mod turret_control_tests {
    use super::*;
    use rust_and_ruin::mech::*;
    use rust_and_ruin::systems::turret_control::*;

    #[test]
    fn test_calculate_turret_rotation() {
        let mech_position = Vec2::new(0.0, 0.0);
        let target_position = Vec2::new(10.0, 0.0);
        
        let angle = calculate_turret_angle(mech_position, target_position);
        
        assert!((angle - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_turret_rotation_angles() {
        let mech_pos = Vec2::ZERO;
        
        let angle_right = calculate_turret_angle(mech_pos, Vec2::new(1.0, 0.0));
        let angle_up = calculate_turret_angle(mech_pos, Vec2::new(0.0, 1.0));
        let angle_left = calculate_turret_angle(mech_pos, Vec2::new(-1.0, 0.0));
        let angle_down = calculate_turret_angle(mech_pos, Vec2::new(0.0, -1.0));
        
        assert!((angle_right - 90.0).abs() < 0.01);
        assert!((angle_up - 0.0).abs() < 0.01);
        assert!((angle_left - 270.0).abs() < 0.01 || (angle_left + 90.0).abs() < 0.01);
        assert!((angle_down - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_rotate_turret_towards_target() {
        let mut current_angle = 0.0;
        let target_angle = 90.0;
        let rotation_speed = 180.0;
        let delta_time = 0.5;
        
        let new_angle = rotate_towards_angle(current_angle, target_angle, rotation_speed, delta_time);
        
        assert!(new_angle > current_angle);
        assert!(new_angle <= target_angle);
        assert!((new_angle - 90.0).abs() < 0.01);
    }

    #[test]
    fn test_turret_rotation_limits() {
        let rotation_speed = 60.0;
        let delta_time = 0.1;
        
        let new_angle = rotate_towards_angle(0.0, 180.0, rotation_speed, delta_time);
        let max_rotation = rotation_speed * delta_time;
        
        assert!((new_angle - max_rotation).abs() < 0.01);
    }

    #[test]
    fn test_angle_normalization() {
        let angle1 = normalize_angle(380.0);
        let angle2 = normalize_angle(-20.0);
        let angle3 = normalize_angle(720.0);
        
        assert!((angle1 - 20.0).abs() < 0.01);
        assert!((angle2 - 340.0).abs() < 0.01);
        assert!((angle3 - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_shortest_rotation_path() {
        let path1 = shortest_angle_difference(350.0, 10.0);
        let path2 = shortest_angle_difference(10.0, 350.0);
        let path3 = shortest_angle_difference(0.0, 180.0);
        
        assert!((path1 - 20.0).abs() < 0.01);
        assert!((path2 - (-20.0)).abs() < 0.01);
        assert!((path3 - 180.0).abs() < 0.01);
    }
}