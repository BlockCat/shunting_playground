use easy_error::{Error, ResultExt};
use model::locations::ShuntingLocations;
use std::io::Read;

pub mod model;

pub use model::locations::*;

pub fn read_locations<R: Read>(reader: R) -> Result<ShuntingLocations, Error> {
    serde_yaml::from_reader(reader).context("Could not parse")
}
