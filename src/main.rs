use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, WorldInspectorPlugin};

mod utils;

#[derive(Inspectable, Default)]
struct Data {
    should_render: bool,
    text: String,
    #[inspectable(min = 42.0, max = 100.0)]
    size: f32,
}

#[derive(Component)]
struct Controllable;

#[derive(Component)]
struct WorldChunk;

#[derive(Component)]
struct TrackingCamera;

#[derive(Component)]
struct TankControllable {
    angle: i32,
}

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<Data>::new())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_startup_system(build_world)
        .add_system(manage_tank_input)
        .add_system(update_ground_chunks)
        .add_system(focus_camera)
        .add_system(focus_tracking_camera)
        .run();
}

fn build_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands
    //     .spawn_bundle(Camera3dBundle {
    //         transform: Transform::from_xyz(15.0, 0.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
    //         ..default()
    //     })
    //     .insert(Name::new("Debug Camera"));
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, -15.0, 6.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
            ..default()
        })
        .insert(TrackingCamera)
        .insert(Name::new("Trailing Camera"));
    const HALF_SIZE: f32 = 50.0;
    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            ..default()
        });
    // Debugging Spheres
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.25, sectors: 15, stacks: 15 })),
            material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Origin Marker"));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.1, sectors: 15, stacks: 15 })),
            material: materials.add(Color::rgb(1.0, 0.1, 0.9).into()),
            transform: Transform::from_xyz(5.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("X Marker"));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.1, sectors: 15, stacks: 15 })),
            material: materials.add(Color::rgb(1.0, 0.1, 0.9).into()),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        })
        .insert(Name::new("Y Marker"));
    
    let mut i = 0;
    let ground_colors = [
        Color::rgb(0.3, 1.0, 0.3),
        Color::rgb(0.9, 0.9, 0.9),
        Color::rgb(0.3, 1.0, 0.3),
        Color::rgb(0.9, 0.9, 0.9),
    ];
    let ground_size = 50.0;
    while i < 4 {
        let mut flat_plane_transform: Transform = Transform::from_xyz(0.0, 0.0, -1.3);
        flat_plane_transform.rotation = Quat::from_rotation_x(utils::deg_to_rad(90.0));
        flat_plane_transform.translation.y += 50.0 * (i as f32);
        flat_plane_transform.translation.z = utils:: calc_world_curve_path(flat_plane_transform.translation.y);
        flat_plane_transform.scale.x = 2.0;
        let name = format!("Ground {}", i + 1);
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Plane { size: ground_size })),
                material: materials.add(ground_colors[i].into()),
                transform: flat_plane_transform,
                ..default()
            })
            .insert(Name::new(name))
            .insert(WorldChunk);

        i += 1;
    }
    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("trashcan.gltf#Scene0"),
            transform: Transform::from_xyz(5.0, 5.0, 0.0),
            ..default()
        })
        .insert(Name::new("trashcan"));
    commands
        .spawn_bundle(SceneBundle {
            scene: asset_server.load("debug.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Debug Boy"))
        .insert(TankControllable { angle: 0 });
}

fn focus_camera(
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    objects: Query<&Transform, (With<TankControllable>, Without<Camera3d>)>
) {
    for mut camera in &mut cameras {
        for obj in &objects {
            camera.rotation = camera.looking_at(obj.translation, Vec3::Z).rotation;
            break; // Still need to figure out how to only access a single object
        }
    }
}

fn manage_tank_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut TankControllable)>
) {
    for (mut transform, mut tank) in query.iter_mut() {
        if keys.pressed(KeyCode::Left) {
            tank.angle += 2;
            if tank.angle > 360 { tank.angle = 0; }
        } else if keys.pressed(KeyCode::Right) {
            tank.angle -= 2;
            if tank.angle < 0 { tank.angle = 360; }
        }
        transform.rotation = Quat::from_rotation_z(utils::deg_to_rad(tank.angle as f32));

        if keys.pressed(KeyCode::Up) {
            // We add 90 to the rotation angle to account for differences in world axis and the orientation of the model.
            let rotation_angle: f32 = tank.angle as f32 + 90.0;
            let speed: f32 = 0.25;
            transform.translation.x += f32::cos(utils::deg_to_rad(rotation_angle)) * speed;
            // transform.translation.y += f32::sin(utils::deg_to_rad(rotation_angle)) * speed;
        }
    }
}

fn focus_tracking_camera(
    tanks: Query<(&mut Transform, &TankControllable), Without<TrackingCamera>>,
    mut cameras: Query<&mut Transform, With<TrackingCamera>>
) {
    
    for mut camera in &mut cameras {
        for (tank_transform, tank_control) in tanks.iter() {
            let rotation_angle: f32 = tank_control.angle as f32 + 270.0;
            let camera_distance: f32 = 15.0;
            camera.translation.x = tank_transform.translation.x + f32::cos(utils::deg_to_rad(rotation_angle)) * camera_distance;
            camera.translation.y = tank_transform.translation.y + f32::sin(utils::deg_to_rad(rotation_angle)) * camera_distance;
            break;
        }
    }
}

fn update_ground_chunks(
    keys: Res<Input<KeyCode>>,
    mut chunks: Query<&mut Transform, With<WorldChunk>>
) {
    if !keys.pressed(KeyCode::Up) { return }
    for mut chunk in &mut chunks {
        chunk.translation.y -= 0.3;
        if chunk.translation.y < -50.0 {
            chunk.translation.y = 150.0;
        }
        chunk.translation.z = utils:: calc_world_curve_path(chunk.translation.y);
    }
}