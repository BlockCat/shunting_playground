use self::shunting_yard::ShuntingYard;
use easy_error::{Error, ResultExt};
use std::io::Read;

pub mod locations;
pub mod shunting_yard;

pub fn read_yard<R: Read>(reader: R) -> Result<ShuntingYard, Error> {
    serde_json::from_reader(reader).context("Could not parse")
}
