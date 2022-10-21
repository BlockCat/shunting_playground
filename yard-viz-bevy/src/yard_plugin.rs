use bevy::prelude::Plugin;
use shunting_location::model::locations::ShuntingLocations;

const locations: &'static str = include_str!("../../data/Kleine_Binckhorst.location.coords.yaml");
const location: &'static str = include_str!("../../data/location.json");

pub struct YardDrawingPlugin;

impl Plugin for YardDrawingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
      
    }
}
