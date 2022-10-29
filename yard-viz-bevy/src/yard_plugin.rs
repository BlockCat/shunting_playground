use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_prototype_lyon::{
    prelude::{DrawMode, GeometryBuilder, StrokeMode},
    shapes,
};
use shunting_location::{read_locations, RailType, ShuntingYard, TrackPart};
use std::io::Cursor;

const LOCATIONS: &'static str = include_str!("../../data/Kleine_Binckhorst.location.coords.yaml");
// const LOCATION: &'static str = include_str!("../../data/location.json");

const SCALE: f32 = 10.0;

pub struct YardDrawingPlugin;

impl Plugin for YardDrawingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(init_shunting_yard);
    }
}

#[derive(Debug, Clone, Component)]
pub struct TrackPartComponent(pub TrackPart);

fn init_shunting_yard(mut commands: Commands) {
    let reader = Cursor::new(LOCATIONS);
    let locations = read_locations(reader).expect("Could not parse");

    let location = Cursor::new(include_str!("../../data/location.json"));
    let yard = ShuntingYard::read(location);

    for part in yard.track_parts() {
        let points = &locations[part]
            .iter()
            .map(|x| Vec2::new(x.x * SCALE, x.y * SCALE))
            .collect::<Vec<_>>();
        if part.kind == RailType::RailRoad {
            let polygon = shapes::Polygon {
                points: points.clone(),
                closed: false,
            };
            commands
                .spawn()
                .insert(Name::new(part.name.clone()))
                .insert(TrackPartComponent(part.clone()))
                .insert_bundle(GeometryBuilder::build_as(
                    &polygon,
                    DrawMode::Stroke(StrokeMode::new(Color::BLACK, 3.0)),
                    Transform::default(),
                ))
                .insert_bundle(PickableBundle::default());
        }
        if part.kind == RailType::Switch
            || part.kind == RailType::EnglishSwitch
            || part.kind == RailType::Intersection
        {
            let circle = shapes::Circle {
                center: points[0].clone(),
                radius: 1.5,
            };
            commands
                .spawn()
                .insert(Name::new(part.name.clone()))
                .insert_bundle(GeometryBuilder::build_as(
                    &circle,
                    DrawMode::Stroke(StrokeMode::new(Color::RED, 1.0)),
                    Transform::from_xyz(0.0, 0.0, 2.0),
                ));
            // dbg!(&locations[part]);
        }

        if part.kind == RailType::Bumper {
            let circle = shapes::Circle {
                center: points[0].clone(),
                radius: 1.5,
            };
            commands
                .spawn()
                .insert(Name::new(part.name.clone()))
                .insert_bundle(GeometryBuilder::build_as(
                    &circle,
                    DrawMode::Stroke(StrokeMode::new(Color::PURPLE, 1.0)),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                ));
        }
    }
}
