use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(PanCamPlugin::default())
            .add_startup_system(init_camera)
            .add_system(camera_controls);
    }
}

fn init_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_scale(Vec3::splat(1.0))
                .with_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..Default::default()
        })
        .insert_bundle(PickingCameraBundle::default())
        .insert(PanCam::default());
}

fn camera_controls(input: Res<Input<KeyCode>>, mut query: Query<(&Camera, &mut Transform)>) {
    let left = input.pressed(KeyCode::A);
    let right = input.pressed(KeyCode::D);
    let up = input.pressed(KeyCode::W);
    let down = input.pressed(KeyCode::S);

    let zoom_in = input.pressed(KeyCode::Q);
    let zoom_out = input.pressed(KeyCode::E);

    query.for_each_mut(|(_, mut transform)| {
        if up {
            transform.translation.y += 1.0;
        }
        if down {
            transform.translation.y -= 1.0;
        }
        if left {
            transform.translation.x -= 1.0;
        }
        if right {
            transform.translation.x += 1.0;
        }
        if zoom_in {
            transform.scale *= 0.95;
        }
        if zoom_out {
            transform.scale *= 1.05;
        }
    });
}
