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
        .add_system(manage_input)
        .run();
}

fn build_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 0.0, 5.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Z),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
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
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 25, stacks: 25 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("trashcan.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }).insert(Name::new("trashcan"));
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
    objects: Query<&Transform, (With<Name>, Without<Camera3d>)>
) {
    for mut camera in &mut cameras {
        for obj in &objects {
            camera.rotation = camera.looking_at(obj.translation, Vec3::Z).rotation;
            break;
        }
    }
}

fn manage_input (
    keys: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform, With<Name>>
){
    for mut transform in &mut transforms {
        if transform.translation.z > 0.0 {
            transform.translation.z -= 0.1;
        }
        if keys.pressed(KeyCode::Left) {
            transform.translation.y -= 0.1;
        } else if keys.pressed(KeyCode::Right) {
            transform.translation.y += 0.1;
        }

        if keys.pressed(KeyCode::Up) {
            transform.translation.x -= 0.1;
        } else if keys.pressed(KeyCode::Down) {
            transform.translation.x += 0.1;
        }

        if keys.pressed(KeyCode::Space) {
            transform.translation.z += 0.69;
        }

        break;
    }
}