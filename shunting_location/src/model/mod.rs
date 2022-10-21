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
    use std::io::Cursor;

    use petgraph::dot::{Config, Dot};

    use crate::{read_locations, shunting_yard::ShuntingYard};

    use super::read_yard;

    const LOCATIONS: &'static str =
        include_str!("../../../data/Kleine_Binckhorst.location.coords.yaml");
    const LOCATION: &'static str = include_str!("../../../data/location.json");

    #[test]
    fn read_locations_test() {
        let r = Cursor::new(LOCATIONS);
        let r = read_locations(r).expect("Could not parse");

        assert_eq!(76, r.0.len());
    }

    #[test]
    fn read_location() {
        let r = Cursor::new(LOCATION);
        read_yard(r).expect("Couuld not parse");
    }

    #[test]
    fn read_yard_location() {
        let yard = read_yard(Cursor::new(LOCATION)).expect("Couuld not parse");
        let yard = ShuntingYard::from(yard);

        let graph = yard.graph;

        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    }
}
