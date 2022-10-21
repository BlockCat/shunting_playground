#![feature(iterator_try_collect)]

use easy_error::{Error, ResultExt};
use model::locations::ShuntingLocations;
use std::io::Read;

pub(crate) mod model;
mod shunting_yard;

pub use model::locations::*;

pub fn read_locations<R: Read>(reader: R) -> Result<ShuntingLocations, Error> {
    serde_yaml::from_reader(reader).context("Could not parse")
}
