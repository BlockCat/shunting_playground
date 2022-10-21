use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(init_camera)
            .add_system(camera_controls);
    }
}

fn init_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        transform: Transform::from_scale(Vec3::splat(0.05)),
        ..Default::default()
    });
}

fn camera_controls() {}
