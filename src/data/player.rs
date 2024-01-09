use binary::datatypes::{Bool, VarI32};
use binary_derive::Binary;

#[derive(Default, Debug, Binary)]
pub struct PlayerMoveSettings {
    pub movement_type: MovementType,
    pub rewind_history_size: VarI32,
    pub server_authoritative_block_breaking: Bool,
}

#[derive(Default, Debug, Binary)]
pub enum MovementType {
    Client,
    Server,
    ServerWithRewind,
    #[default]
    Invalid,
}
