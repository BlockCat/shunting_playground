#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Train {
    units: Vec<TrainUnit>,
    direction: TrainDirection,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrainUnit(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrainDirection {
    SideA,
    SideB,
}
