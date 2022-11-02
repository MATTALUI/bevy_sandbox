use bevy::prelude::*;

mod utils;

pub fn focus_camera(
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

pub fn manage_tank_input(
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
            transform.translation.y += f32::sin(utils::deg_to_rad(rotation_angle)) * speed;
        }
    }
}