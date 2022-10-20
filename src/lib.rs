use std::ops::Deref;

pub mod yard;
pub mod solution;
pub mod train;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RailId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RailSwitchId(pub usize);

impl Deref for RailId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RailSwitchId {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[derive(Debug, Clone)]
pub struct ServiceType(String);


