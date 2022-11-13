/// Train
/// Train does not need a direction, because the node it's at determines said direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Train {
    units: Vec<TrainUnit>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrainUnit(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrainDirection {
    SideA,
    SideB,
}
