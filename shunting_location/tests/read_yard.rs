use shunting_location::{model::read_yard, read_locations};
use std::io::Cursor;

const locations: &'static str = include_str!("../../data/Kleine_Binckhorst.location.coords.yaml");
const location: &'static str = include_str!("../../data/location.json");

#[test]
fn read_locations_test() {
    let r = Cursor::new(locations);
    let r = read_locations(r).expect("Could not parse");

    assert_eq!(76, r.0.len());
}

#[test]
fn read_location() {
    let r = Cursor::new(location);
    let r = read_yard(r).expect("Couuld not parse");
}
