use std::time::{Duration, Instant, SystemTime};

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_prototype_lyon::prelude::*;
use camera_plugin::CameraPlugin;
use rail_view::RailViewPlugin;
mod camera_plugin;
mod rail_view;
mod yard_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(yard_plugin::YardDrawingPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(RailViewPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Time")
        .default_width(5000.0)
        .min_width(5000.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(
                egui::Slider::new(&mut 0u64, 0..=24 * 60 * 60u64).custom_formatter(|d: f64, _| {
                    let duration = Duration::from_secs(d as u64);
                    format!(
                        "{}:{}:{}",
                        duration.as_secs() / 3600,
                        (duration.as_secs() % 3600) / 60,
                        duration.as_secs() % 60
                    )
                }),
            );
        });
}
