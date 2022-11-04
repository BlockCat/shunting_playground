use std::io::Cursor;

use petgraph::dot::{Config, Dot};
use shunting_location::ShuntingYard;

const LOCATION: &str = include_str!("../../data/location.json");

fn main() {    
    let yard = ShuntingYard::read(Cursor::new(LOCATION));
    let graph = yard.graph;
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
}
