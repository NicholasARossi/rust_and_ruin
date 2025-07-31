use bevy::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::render::RenderPlugin;

#[test]
fn test_material_colors_render_correctly() {
    let mut app = App::new();
    
    app.add_plugins((
        MinimalPlugins,
        AssetPlugin::default(),
        RenderPlugin::default(),
        ImagePlugin::default(),
    ));
    
    // Initialize material assets
    app.init_asset::<StandardMaterial>();
    app.init_asset::<Mesh>();
    
    // Add required resources
    let _meshes = app.world.resource_mut::<Assets<Mesh>>();
    let mut materials = app.world.resource_mut::<Assets<StandardMaterial>>();
    
    // Test creating materials with different colors
    let gray_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.3, 0.3),
        unlit: true,
        ..default()
    });
    
    let green_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.6, 0.0),
        unlit: true,
        ..default()
    });
    
    let red_material = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 0.0, 0.0),
        unlit: true,
        ..default()
    });
    
    // Verify materials were created
    assert!(materials.get(&gray_material).is_some());
    assert!(materials.get(&green_material).is_some());
    assert!(materials.get(&red_material).is_some());
    
    // Verify colors are set correctly
    let gray = materials.get(&gray_material).unwrap();
    assert_eq!(gray.base_color, Color::rgb(0.3, 0.3, 0.3));
    assert_eq!(gray.unlit, true);
    
    let green = materials.get(&green_material).unwrap();
    assert_eq!(green.base_color, Color::rgb(0.0, 0.6, 0.0));
    
    let red = materials.get(&red_material).unwrap();
    assert_eq!(red.base_color, Color::rgb(1.0, 0.0, 0.0));
}

#[test]
fn test_camera_with_tonemapping_none() {
    let mut app = App::new();
    
    app.add_plugins((
        MinimalPlugins,
        TransformPlugin,
        HierarchyPlugin,
    ));
    
    // Spawn camera with tonemapping set to None
    let camera_entity = app.world.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 0.1,
            ..default()
        }.into(),
        tonemapping: Tonemapping::None,
        ..default()
    }).id();
    
    // Verify camera was created with correct settings
    let camera = app.world.get::<Camera>(camera_entity);
    assert!(camera.is_some());
    
    let tonemapping = app.world.get::<Tonemapping>(camera_entity);
    assert!(tonemapping.is_some());
    assert_eq!(*tonemapping.unwrap(), Tonemapping::None);
}

#[test]
fn test_clear_color_setting() {
    let mut app = App::new();
    
    app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));
    
    let clear_color = app.world.resource::<ClearColor>();
    assert_eq!(clear_color.0, Color::rgb(0.1, 0.1, 0.1));
}