use std::collections::HashSet;

/// Train
/// Train does not need a direction, because the node it's at determines said direction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Train {
    units: Vec<TrainUnit>,
}

impl Train {
    pub fn new(units: &[usize]) -> Train {
        Self {
            units: units.iter().map(|x| TrainUnit(*x)).collect::<Vec<_>>(),
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        for a in &self.units {
            for b in &other.units {
                if a == b {
                    return true;
                }
            }
        }
        return false;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TrainUnit(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrainDirection {
    SideA,
    SideB,
}
