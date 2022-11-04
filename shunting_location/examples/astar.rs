use std::io::Cursor;

use shunting_location::{ShuntingSwitch, ShuntingYard};

const LOCATION: &str = include_str!("../../data/location.json");

fn main() {
    let yard = ShuntingYard::read(Cursor::new(LOCATION));

    let graph = yard.graph;

    let l = petgraph::algo::astar(
        &graph,
        petgraph::stable_graph::NodeIndex::new(31),
        |d| d == petgraph::stable_graph::NodeIndex::new(25),
        |f| {
            println!("S: {:?}", f);
            match f.weight() {
                ShuntingSwitch::Rotation => 30.0f32,
                ShuntingSwitch::Switch => yard.movement_switch_coefficient,
                _ => unreachable!(),
            }
        },
        |_| 1.0,
    );

    if let Some((weight, path)) = l {
        println!("Weight: {}", weight);
        for node in path {
            println!("{:?}", graph[node]);
        }
    } else {
        unreachable!();
    }
}
