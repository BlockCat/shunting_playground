use petgraph::{prelude::DiGraph, Graph};
use crate::{RailId, RailSwitchId, ServiceType};

pub type LayoutGraph = DiGraph<NodeAttribute, EdgeAttribute, RailLocationIndex>;

#[derive(Debug, Clone)]
pub struct RailLocationIndex(u32);

/**
 * The layout of the shunting yard,
 * Node is a rail + direction,
 * Edge is a switch
 */
pub struct YardLayout {
    graph: LayoutGraph,
}

impl YardLayout {
    pub fn new(graph: DiGraph<NodeAttribute, EdgeAttribute, RailLocationIndex>) -> Self {
        Self { graph }
    }
}

/**
 * Nodes are rails
 */
pub struct NodeAttribute {
    pub name: String,
    pub can_stop: bool,
    pub provide_service: Option<ServiceType>,
}
pub struct EdgeAttribute {
    pub name: String,
}
