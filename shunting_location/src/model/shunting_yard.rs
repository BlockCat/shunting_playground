use serde::{de, Deserialize};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct TrackPartYamlId(usize);

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShuntingYardYaml {
    #[serde(rename = "trackParts")]
    pub track_parts: Vec<TrackPart>,

    pub facilities: Vec<Facility>,

    #[serde(rename = "taskTypes")]
    pub task_types: Vec<TaskType>,

    #[serde(rename = "movementConstant")]
    pub movement_constant: f32,
    #[serde(rename = "movementTrackCoefficient")]
    pub movement_track_coefficient: f32,
    #[serde(rename = "movementSwitchCoefficient")]
    pub movement_switch_coefficient: f32,

    #[serde(rename = "distanceEntries")]
    pub distance_entries: Vec<DistanceEntry>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct TrackPart {
    #[serde(deserialize_with = "parse_string")]
    pub id: TrackPartYamlId,
    #[serde(rename = "type")]
    pub kind: RailType,
    #[serde(rename = "aSide", deserialize_with = "parse_vec_string")]
    pub a_side: Vec<TrackPartYamlId>,
    #[serde(rename = "bSide", deserialize_with = "parse_vec_string")]
    pub b_side: Vec<TrackPartYamlId>,
    pub length: f32,
    pub name: String,
    #[serde(rename = "sawMovementAllowed")]
    pub saw_movement_allowed: bool,
    #[serde(rename = "parkingAllowed")]
    pub parking_allowed: bool,
    #[serde(rename = "isElectrified")]
    pub is_electrified: bool,
    #[serde(rename = "aSideOpen")]
    pub a_side_open: bool,
    #[serde(rename = "bSideOpen")]
    pub b_side_open: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum RailType {
    RailRoad,
    Bumper,
    Intersection,
    Switch,
    EnglishSwitch,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct Facility {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: FacilityType,
    #[serde(rename = "relatedTrackParts", deserialize_with = "parse_vec_string")]
    pub related_track_parts: Vec<TrackPartYamlId>,
    #[serde(rename = "taskTypes")]
    pub task_types: Vec<TaskType>,
    #[serde(rename = "simultaneousUsageCount")]
    pub capacity: usize,
}
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub enum FacilityType {
    ElevatedPlatform,
    Pit,
    CleaningPlatform,
    WashingMachine,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
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

fn parse_string<'de, D>(deserializer: D) -> Result<TrackPartYamlId, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    value
        .parse()
        .map_err(|_| {
            de::Error::invalid_value(de::Unexpected::Str(&value), &"String parseable to number")
        })
        .map(TrackPartYamlId)
}

fn parse_vec_string<'de, D>(deserializer: D) -> Result<Vec<TrackPartYamlId>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let value: Vec<String> = Vec::deserialize(deserializer)?;
    value
        .iter()
        .map(|value| value.parse::<usize>())
        .map(|value| value.map(TrackPartYamlId))
        .try_collect::<Vec<_>>()
        .map_err(de::Error::custom)
}
