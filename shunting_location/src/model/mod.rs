use self::shunting_yard::ShuntingYardYaml;
use easy_error::{Error, ResultExt};
use std::io::Read;

pub mod locations;
pub mod shunting_yard;

pub fn read_yard<R: Read>(reader: R) -> Result<ShuntingYardYaml, Error> {
    serde_json::from_reader(reader).context("Could not parse")
}

#[cfg(test)]
mod tests {
    use super::read_yard;
    use crate::read_locations;
    use std::io::Cursor;

    const LOCATIONS: &str =
        include_str!("../../../data/Kleine_Binckhorst.location.coords.yaml");
    const LOCATION: &str = include_str!("../../../data/location.json");

    #[test]
    fn read_locations_test() {
        let r = Cursor::new(LOCATIONS);
        let r = read_locations(r).expect("Could not parse");

        assert_eq!(76, r.0.len());
    }

    #[test]
    fn read_location() {
        let r = Cursor::new(LOCATION);
        read_yard(r).expect("Could not parse");
    }
}
