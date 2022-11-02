use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, WorldInspectorPlugin};

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
        .add_system(focus_camera)
        .add_system(manage_tank_input)
        .run();
}

fn build_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(15.0, 0.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
            ..default()
        });
    const HALF_SIZE: f32 = 25.0;
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
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.25, sectors: 15, stacks: 15 })),
            material: materials.add(Color::rgb(1.0, 0.1, 0.1).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Name::new("Origin Marker"));

    let mut flat_plane_transform: Transform = Transform::from_xyz(0.0, 0.0, -1.3);
    flat_plane_transform.rotation = Quat::from_rotation_x(deg_to_rad(90.0));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.3, 1.0, 0.3).into()),
            transform: flat_plane_transform,
            ..default()
        })
        .insert(Name::new("Ground"));
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

// fn animate_light_direction(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<DirectionalLight>>,
// ) {
//     for mut transform in &mut query {
//         transform.rotation = Quat::from_euler(
//             EulerRot::ZYX,
//             0.0,
//             time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
//             -std::f32::consts::FRAC_PI_4,
//         );
//     }
// }
    
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

// fn manage_input (
//     keys: Res<Input<KeyCode>>,
//     mut transforms: Query<&mut Transform, With<Controllable>>
// ){
//     for mut transform in &mut transforms {
//         if transform.translation.z > 0.0 {
//             transform.translation.z -= 0.1;
//         }
//         if keys.pressed(KeyCode::Left) {
//             transform.translation.y -= 0.1;
//         } else if keys.pressed(KeyCode::Right) {
//             transform.translation.y += 0.1;
//         }

//         if keys.pressed(KeyCode::Up) {
//             transform.translation.x -= 0.1;
//         } else if keys.pressed(KeyCode::Down) {
//             transform.translation.x += 0.1;
//         }

//         if keys.pressed(KeyCode::Space) {
//             transform.translation.z += 0.69;
//         }
//     }
// }

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
        transform.rotation = Quat::from_rotation_z(deg_to_rad(tank.angle as f32));

        if keys.pressed(KeyCode::Up) {
            let speed: f32 = 0.25;
            let rotation_angle: f32 = tank.angle as f32 + 90.0;
            transform.translation.x += f32::cos(deg_to_rad(rotation_angle)) * speed;
            transform.translation.y += f32::sin(deg_to_rad(rotation_angle)) * speed;
        }
    }
}

fn deg_to_rad(degrees: f32) -> f32 {
    return degrees * (std::f32::consts::PI / 180.0);
}