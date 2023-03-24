use crate::structs::light_types::*;

#[derive(Debug, Default)]
pub struct LightManager{
    strips: Vec<LightStrip>,
    bulbs: Vec<Bulb>,
    bulb_groups: Vec<BulbGroup>
} 