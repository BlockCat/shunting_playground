use std::io::Cursor;

use shunting_location::{ShuntingSwitch, ShuntingYard};

const LOCATION: &'static str = include_str!("../../data/location.json");

fn main() {
    let yard = ShuntingYard::read(Cursor::new(LOCATION));

    let graph = yard.graph;

    let l = petgraph::algo::astar(
        &graph,
        petgraph::stable_graph::NodeIndex::new(30),
        |d| d == petgraph::stable_graph::NodeIndex::new(25),
        |f| match f.weight() {
            ShuntingSwitch::Rotation => 30,
            ShuntingSwitch::Switch => 60,
            _ => unreachable!(),
        },
        |f| 1,
    );

    if let Some((weight, path)) = l {
        println!("Weight: {}", weight);
        for node in path {
            println!("{:?}", graph[node]);
        }
    }
}
