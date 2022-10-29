use super::shunting_yard::TrackPart;
use serde::Deserialize;
use std::{collections::HashMap, ops::Index};

#[derive(Debug, PartialEq, Deserialize)]
pub struct LocationCoord {
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShuntingLocations(pub HashMap<String, Vec<LocationCoord>>);

impl Index<&TrackPart> for ShuntingLocations {
    type Output = Vec<LocationCoord>;

    fn index(&self, index: &TrackPart) -> &Self::Output {
        &self.0[&index.name]
    }
}

impl Index<String> for ShuntingLocations {
    type Output = Vec<LocationCoord>;

    fn index(&self, index: String) -> &Self::Output {
        &self.0[&index]
    }
}
