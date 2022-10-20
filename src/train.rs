#[derive(Debug, Clone)]
pub struct Train {
    units: Vec<TrainUnit>,
    length: f32,
}

impl Train {
    pub fn new(units: Vec<TrainUnit>) -> Self {
        Self {
            length: units.iter().map(|f| f.length).sum(),
            units,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrainUnit {
    length: f32,
}
