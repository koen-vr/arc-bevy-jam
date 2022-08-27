use super::*;

#[derive(Component, Clone, Copy, Debug, Hash, Inspectable)]
pub struct HealthRecource {
    pub value: u16,
}

#[derive(Component, Clone, Copy, Debug, Hash, Inspectable)]
pub struct EnergyRecource {
    pub value: u16,
}
