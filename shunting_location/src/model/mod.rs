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

    use petgraph::{
        dot::{Config, Dot},
        stable_graph::NodeIndex,
        Graph,
    };

    use crate::{read_locations, shunting_yard::ShuntingYard, ShuntingSwitch};

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
        read_yard(r).expect("Could not parse");
    }

    


    #[test]
    fn yard_all_simple_paths() {
        let yard = ShuntingYard::read(Cursor::new(LOCATION));
        let graph = yard.graph;

        let paths = petgraph::algo::all_simple_paths::<Vec<_>, _>(
            &graph,
            petgraph::stable_graph::NodeIndex::new(30),
            petgraph::stable_graph::NodeIndex::new(25),
            0,
            None,
        )
        .collect::<Vec<_>>();

        let mut buffer = String::new();

        for path in &paths {
            for node in path {
                buffer += &format!("{:?}\n", graph[*node]);
            }
            buffer += "----------------------\n";
        }
        println!("{} Found: {} paths", buffer, paths.len());
    }
}
