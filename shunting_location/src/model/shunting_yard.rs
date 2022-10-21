use serde::{
    de::{self, Error},
    Deserialize,
};
use std::time::Duration;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShuntingYard {
    #[serde(rename = "trackParts")]
    pub track_parts: Vec<TrackPart>,

    facilities: Vec<Facility>,

    #[serde(rename = "taskTypes")]
    task_types: Vec<TaskType>,

    #[serde(rename = "movementConstant")]
    movement_constant: f32,
    #[serde(rename = "movementTrackCoefficient")]
    movement_track_coefficient: f32,
    #[serde(rename = "movementSwitchCoefficient")]
    movement_switch_coefficient: f32,

    #[serde(rename = "distanceEntries")]
    distance_entries: Vec<DistanceEntry>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TrackPart {
    id: String,
    #[serde(rename = "type")]
    kind: RailType,
    #[serde(rename = "aSide")]
    a_side: Vec<String>,
    #[serde(rename = "bSide")]
    b_side: Vec<String>,
    length: f32,
    pub name: String,
    #[serde(rename = "sawMovementAllowed")]
    saw_movement_allowed: bool,
    #[serde(rename = "parkingAllowed")]
    parking_allowed: bool,
    #[serde(rename = "isElectrified")]
    is_electrified: bool,
    #[serde(rename = "aSideOpen")]
    a_side_open: bool,
    #[serde(rename = "bSideOpen")]
    b_side_open: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum RailType {
    RailRoad,
    Bumper,
    Intersection,
    Switch,
    EnglishSwitch,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Facility {
    id: String,
    #[serde(rename = "type")]
    kind: FacilityType,
    #[serde(rename = "relatedTrackParts")]
    related_track_parts: Vec<String>,
    #[serde(rename = "taskTypes")]
    task_types: Vec<TaskType>,
    #[serde(rename = "simultaneousUsageCount")]
    capacity: usize,
}
#[derive(Debug, PartialEq, Deserialize)]
pub enum FacilityType {
    ElevatedPlatform,
    Pit,
    CleaningPlatform,
    WashingMachine,
    Unknown,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TaskType {
    other: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct DistanceEntry {
    #[serde(rename = "fromTrackPartId")]
    from: String,
    #[serde(rename = "toTrackPartId")]
    to: String,
    #[serde(rename = "distanceInSeconds", deserialize_with = "distance_duration")]
    distance: Duration,
}

fn distance_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: de::Deserializer<'de>,
{
    f32::deserialize(deserializer).map(Duration::from_secs_f32)
}
