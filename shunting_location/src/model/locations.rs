use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct LocationCoord {
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShuntingLocations(pub HashMap<String, Vec<LocationCoord>>);
