use binary_derive::Binary;

#[derive(Debug, Default, Binary)]
#[data(datatype = "I32")]
pub enum MovementMode {
    Normal,
    Reset,
    Teleportation,
    Rotation,
    #[default]
    Invalid,
}

#[derive(Debug, Default, Binary)]
#[data(datatype = "U8")]
pub enum TeleportCause {
    #[default]
    Unknown,
    Projectile,
    ChorusFruit,
    Command,
    Behaviour,
}
