use binary::datatypes::{F32, I64};
use binary_derive::Binary;
use byteorder::LE;

pub const BASE_FLY_SPEED: f32 = 0.05;
pub const BASE_WALK_SPEED: f32 = 0.1;

#[derive(Debug, Default, Binary)]
pub struct AbilityData {
    pub entity_unique_id: I64<LE>,
}

#[derive(Debug, Default, Binary)]
pub struct AbilityLayer {
    pub layer_type: AbilityLayerType,
    pub fly_speed: F32<LE>,
    pub walk_speed: F32<LE>,
}

#[derive(Debug, Default, Binary)]
#[data(datatype = "U16")]
pub enum AbilityLayerType {
    Cache,
    Base,
    Spectator,
    Commands,
    Editor,
    #[default]
    Invalid,
}
