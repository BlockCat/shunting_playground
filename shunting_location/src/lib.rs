#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use easy_error::{Error, ResultExt};
use std::io::Read;

pub use model::locations::LocationCoord;
pub use model::locations::ShuntingLocations;
pub use model::shunting_yard::{DistanceEntry, Facility, RailType, TrackPart};
pub use shunting_yard::{rail::ShuntingRail, switch::ShuntingSwitch, ShuntingYard};
pub(crate) mod model;
mod shunting_yard;

pub fn read_locations<R: Read>(reader: R) -> Result<ShuntingLocations, Error> {
    serde_yaml::from_reader(reader).context("Could not parse")
}
