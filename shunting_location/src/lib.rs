#![feature(iterator_try_collect)]
#![feature(is_sorted)]

use easy_error::{Error, ResultExt};
use model::locations::ShuntingLocations;
use std::io::Read;

pub(crate) mod model;
mod shunting_yard;

pub use model::locations::*;
pub use shunting_yard::{rail::ShuntingRail, switch::ShuntingSwitch, ShuntingYard};

pub fn read_locations<R: Read>(reader: R) -> Result<ShuntingLocations, Error> {
    serde_yaml::from_reader(reader).context("Could not parse")
}
