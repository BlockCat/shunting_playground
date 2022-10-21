use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use camera_plugin::CameraPlugin;
use shunting_location::model::{locations::ShuntingLocations, read_yard};
use std::io::Cursor;

mod camera_plugin;
mod yard_plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(yard_plugin::YardDrawingPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(init_shunting_yard)
        .run();
}

fn init_shunting_yard(mut commands: Commands) {
    let locations = Cursor::new(include_str!(
        "../../data/Kleine_Binckhorst.location.coords.yaml"
    ));
    let locations: ShuntingLocations =
        shunting_location::read_locations(locations).expect("Could not parse");

    let location = Cursor::new(include_str!("../../data/location.json"));
    let yard = read_yard(location).expect("Could not read yard");

    for part in &yard.track_parts {
        let name = &part.name;
        let line = &locations.0[name];

        for w in line.windows(2) {
            let line = shapes::Line(Vec2::new(w[0].x, w[0].y), Vec2::new(w[1].x, w[1].y));
            commands.spawn_bundle(GeometryBuilder::build_as(
                &line,
                DrawMode::Stroke(StrokeMode::new(Color::BLACK, 0.3)),
                Transform::default(),
            ));
        }
    }

    commands.insert_resource(locations);
    commands.insert_resource(yard);
}
