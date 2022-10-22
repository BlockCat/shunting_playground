use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_mod_picking::{HoverEvent, PickingEvent};
use shunting_location::TrackPart;

use crate::yard_plugin::TrackPartComponent;

pub struct RailViewPlugin;

impl Plugin for RailViewPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RailViewModel::default())
            .add_system(handle_events)
            .add_system(render_view);
    }
}

#[derive(Debug, Default)]
pub struct RailViewModel {
    selected: Option<TrackPart>,
}
fn handle_events(
    mut events: EventReader<PickingEvent>,
    mut rail_data: ResMut<RailViewModel>,
    query: Query<&TrackPartComponent>,
) {
    for event in events.iter() {
        match event {
            PickingEvent::Hover(HoverEvent::JustEntered(e)) => {
                let data = query.get(*e).expect("Could not select part");
                rail_data.selected = Some(data.0.clone());
            }
            _ => {}
        }
    }
}

fn render_view(mut egui_context: ResMut<EguiContext>, rail_data: Res<RailViewModel>) {
    bevy_egui::egui::Window::new("Rail data").show(egui_context.ctx_mut(), |ui| {
        if let Some(data) = &rail_data.selected {
            ui.label(format!("Name: {}", data.name));
            ui.label(format!("Length: {}m", data.length));

            if data.parking_allowed {
                ui.label("Is allowed to park");
            }
            if data.saw_movement_allowed {
                ui.label("Is allowed to turn");
            }
        }
    });
}
