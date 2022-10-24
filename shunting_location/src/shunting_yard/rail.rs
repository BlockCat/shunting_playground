use crate::TrackPart;

use super::facility::FacilityId;

#[derive(Debug, Clone)]
pub struct ShuntingRail {
    /// Yaml id, used for shared locked
    pub id: String,
    pub length: f32,
    pub saw_movement_allowed: bool,
    pub parking_allowed: bool,
    pub facilities: Vec<FacilityId>,
    pub track_part: TrackPart,
}
