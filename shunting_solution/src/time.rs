use std::ops::Add;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ShuntingTime(usize);

impl ShuntingTime {
    pub fn new(seconds: usize) -> Self {
        Self(seconds)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ShuntingDuration(usize);

impl ShuntingDuration {
    pub fn new(seconds: usize) -> Self {
        Self(seconds)
    }
}

impl Add<ShuntingDuration> for ShuntingTime {
    type Output = ShuntingTime;

    fn add(self, rhs: ShuntingDuration) -> Self::Output {
        ShuntingTime(self.0 + rhs.0)
    }
}

impl Add<ShuntingDuration> for ShuntingDuration {
    type Output = ShuntingDuration;

    fn add(self, rhs: ShuntingDuration) -> Self::Output {
        ShuntingDuration(self.0 + rhs.0)
    }
}
