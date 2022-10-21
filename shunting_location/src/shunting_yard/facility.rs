#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct FacilityId(pub(crate) u8);


pub struct Facility {
    pub id: FacilityId,
    pub name: String,
}
