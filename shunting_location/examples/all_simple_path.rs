use std::io::Cursor;

use shunting_location::ShuntingYard;

const LOCATION: &str = include_str!("../../data/location.json");

fn main() {
    let yard = ShuntingYard::read(Cursor::new(LOCATION));
    let graph = yard.graph;

    let paths = petgraph::algo::all_simple_paths::<Vec<_>, _>(
        &graph,
        petgraph::stable_graph::NodeIndex::new(31),
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
