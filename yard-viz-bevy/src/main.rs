use bevy::{prelude::*, utils::Instant};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_prototype_lyon::prelude::*;
use camera_plugin::CameraPlugin;
use dipstick::{Input, *};
use rail_view::RailViewPlugin;
use std::time::Duration;

mod camera_plugin;
mod rail_view;
mod yard_plugin;

fn main() {
    let stats_connection = Statsd::send_to("localhost:8125").map(|x| {
        let stat = x.named("bevy_yard_viz");
        stat.prefix_prepend("name");
        stat.metrics()
    });

    if let Ok(stat_connection) = stats_connection {
        let name = format!("bevy_viz");
        Proxy::default().named(name);
        dipstick::Proxy::default_target(stat_connection);
    } else {
        panic!("Not connected");
    }

    // dipstick::Proxy::default_target(
    //     Stream::write_to_stdout().metrics(), // Graphite::send_to("localhost:2003")
    //                                          //     .expect("Could not connect")
    //                                          //     .named("My_app_how")
    //                                          //     .metrics(),
    // );
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
    egui::Window::new("Time").show(egui_context.ctx_mut(), |ui| {
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
