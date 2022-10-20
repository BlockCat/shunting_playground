use petgraph::prelude::*;

use shunting_playground::yard::layout::{
    EdgeAttribute, LayoutGraph, NodeAttribute, RailLocationIndex, YardLayout,
};

pub fn super_basic_yard() {
    let mut graph = DiGraph::default();

    let rail1 = add_rail(
        &mut graph,
        NodeAttribute {
            name: String::from("1"),
            can_stop: false,
            provide_service: None,
        },
        EdgeAttribute {
            name: String::from("t1"),
        },
    );
    let rail2 = add_rail(
        &mut graph,
        NodeAttribute {
            name: String::from("2"),
            can_stop: false,
            provide_service: None,
        },
        EdgeAttribute {
            name: String::from("t2"),
        },
    );
    let rail3 = add_rail(
        &mut graph,
        NodeAttribute {
            name: String::from("3"),
            can_stop: false,
            provide_service: None,
        },
        EdgeAttribute {
            name: String::from("t3"),
        },
    );
    let rail4 = add_rail(
        &mut graph,
        NodeAttribute {
            name: String::from("4"),
            can_stop: false,
            provide_service: None,
        },
        EdgeAttribute {
            name: String::from("t4"),
        },
    );
    let s1 = EdgeAttribute {
        name: String::from("s1")
    };
    add_connection(&mut graph, &rail1, &rail2, s1.clone());
    add_connection(&mut graph, &rail1, &rail2, s1);

    let yard = YardLayout::new(graph);
}

fn add_connection(graph: &mut LayoutGraph, a: &RailResult, b: &RailResult, edge: EdgeAttribute) {
    graph.add_edge(a.a, b.a, edge.clone());
    graph.add_edge(b.b, a.b, edge);
}

fn add_rail(
    graph: &mut DiGraph<NodeAttribute, EdgeAttribute, RailLocationIndex>,
    weight: NodeAttribute,
    switch: EdgeAttribute,
) -> RailResult {
    let a = graph.add_node(weight.clone());
    let b = graph.add_node(weight);

    graph.add_edge(a, b, switch.clone());
    graph.add_edge(b, a, switch);

    (a, b)
}

struct RailResult {
    a: NodeIndex<RailLocationIndex>,
    b: NodeIndex<RailLocationIndex>,
}
