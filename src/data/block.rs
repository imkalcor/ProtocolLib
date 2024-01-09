use binary_derive::Binary;
use nbt::{encoding::NetworkLittleEndian, NBTCompound};

use super::VarString;

#[derive(Default, Debug, Binary)]
pub struct BlockEntry {
    pub name: VarString,
    pub properties: NBTCompound<NetworkLittleEndian>,
}
