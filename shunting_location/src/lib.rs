#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use easy_error::{Error, ResultExt};
use std::io::Read;

pub use model::locations::LocationCoord;
pub use model::locations::ShuntingLocations;
pub use model::shunting_yard::{DistanceEntry, RailType, TrackPart};
pub use shunting_yard::{
    facility::{Facility, FacilityId},
    rail::ShuntingRail,
    switch::ShuntingSwitch,
    ShuntingYard, YardGraph, YardGraphIndex,
};

pub(crate) mod model;
mod shunting_yard;

pub fn read_locations<R: Read>(reader: R) -> Result<ShuntingLocations, Error> {
    serde_yaml::from_reader(reader).context("Could not parse")
}
