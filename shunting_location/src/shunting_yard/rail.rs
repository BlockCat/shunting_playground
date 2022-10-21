use super::facility::FacilityId;

#[derive(Debug, Clone)]
pub enum ShuntingRail {
    Rail {
        /// Yaml id, used for shared locked
        id: String,
        length: f32,
        saw_movement_allowed: bool,
        parking_allowed: bool,
        facilities: Vec<FacilityId>,
    },
    Bumper {
        id: String,        
    },
    Entry {
        id: String
    }
}
