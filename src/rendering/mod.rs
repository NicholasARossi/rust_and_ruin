use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MechPartType {
    TankTreads,
    TurretBase,
    CannonBarrel,
}

pub fn create_sprite_mesh(size: Vec2) -> Mesh {
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;
    
    let vertices = vec![
        [-half_width, 0.0, -half_height],
        [half_width, 0.0, -half_height],
        [half_width, 0.0, half_height],
        [-half_width, 0.0, half_height],
    ];
    
    let normals = vec![[0.0, 1.0, 0.0]; 4];
    let uvs = vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]];
    let indices = vec![0, 1, 2, 2, 3, 0];
    
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    
    mesh
}

pub fn create_sprite_material(color: Color, materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    })
}

pub fn create_tank_tread_mesh(size: Vec2) -> Mesh {
    let half_width = size.x / 2.0;
    let half_height = size.y / 2.0;
    
    let vertices = vec![
        [-half_width, 0.0, -half_height],
        [-half_width * 0.6, 0.0, -half_height],
        [half_width * 0.6, 0.0, -half_height],
        [half_width, 0.0, -half_height],
        [half_width, 0.0, half_height],
        [half_width * 0.6, 0.0, half_height],
        [-half_width * 0.6, 0.0, half_height],
        [-half_width, 0.0, half_height],
    ];
    
    let normals = vec![[0.0, 1.0, 0.0]; 8];
    let uvs = vec![
        [0.0, 0.0], [0.2, 0.0], [0.8, 0.0], [1.0, 0.0],
        [1.0, 1.0], [0.8, 1.0], [0.2, 1.0], [0.0, 1.0],
    ];
    let indices = vec![
        0, 1, 7, 1, 6, 7,
        1, 2, 6, 2, 5, 6,
        2, 3, 5, 3, 4, 5,
    ];
    
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    
    mesh
}

pub fn create_turret_base_mesh(radius: f32) -> Mesh {
    let segments = 16;
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 1.0, 0.0]];
    let mut uvs = vec![[0.5, 0.5]];
    
    for i in 0..segments {
        let angle = (i as f32 / segments as f32) * 2.0 * PI;
        let x = radius * angle.cos();
        let z = radius * angle.sin();
        vertices.push([x, 0.0, z]);
        normals.push([0.0, 1.0, 0.0]);
        
        let u = 0.5 + 0.5 * angle.cos();
        let v = 0.5 + 0.5 * angle.sin();
        uvs.push([u, v]);
    }
    
    let mut indices = Vec::new();
    for i in 0..segments {
        let next = if i == segments - 1 { 1 } else { i + 2 };
        indices.extend_from_slice(&[0, i + 1, next]);
    }
    
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    
    mesh
}

pub fn create_cannon_barrel_mesh(length: f32, width: f32) -> Mesh {
    let half_width = width / 2.0;
    
    let vertices = vec![
        [-half_width, 0.0, 0.0],
        [half_width, 0.0, 0.0],
        [half_width, 0.0, length],
        [-half_width, 0.0, length],
    ];
    
    let normals = vec![[0.0, 1.0, 0.0]; 4];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let indices = vec![0, 1, 2, 2, 3, 0];
    
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U32(indices)));
    
    mesh
}

pub fn create_mech_material(part_type: MechPartType, materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    let color = match part_type {
        MechPartType::TankTreads => Color::rgb(0.4, 0.4, 0.4),
        MechPartType::TurretBase => Color::rgb(0.2, 0.6, 1.0),
        MechPartType::CannonBarrel => Color::rgb(0.7, 0.7, 0.7),
    };
    
    materials.add(StandardMaterial {
        base_color: color,
        unlit: true,
        alpha_mode: AlphaMode::Opaque,
        ..default()
    })
}

pub fn get_mech_part_offset(part_type: MechPartType) -> Vec3 {
    match part_type {
        MechPartType::TankTreads => Vec3::ZERO,
        MechPartType::TurretBase => Vec3::new(0.0, 0.1, 0.0),
        MechPartType::CannonBarrel => Vec3::new(0.0, 0.0, 0.3),
    }
}