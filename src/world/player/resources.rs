use super::*;

#[derive(Component, Clone, Copy, Debug, Hash, Inspectable)]
pub struct HealthRecource {
    pub max: u16,
    pub value: u16,
}

#[derive(Component, Clone, Copy, Debug, Hash, Inspectable)]
pub struct EnergyRecource {
    pub max: u16,
    pub value: u16,
}
