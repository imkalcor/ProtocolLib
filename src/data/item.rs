use bytes::Buf;
use std::io::{Error, ErrorKind, Read, Result, Write};

use binary::{
    binary::Binary,
    datatypes::{Bool, VarI32, VarU32, I16, U16},
};
use binary_derive::Binary;
use byteorder::LE;
use nbt::{encoding::LittleEndian, NBTCompound};

use super::{SliceU32, StringUTF, VarString};

#[derive(Default, Debug, Binary)]
pub struct ItemInstance {
    pub stack_network_id: Option<VarI32>,
    pub stack: ItemStack,
}

#[derive(Default, Debug)]
pub struct ItemStack {
    pub item_type: ItemType,
    pub block_runtime_id: VarI32,
    pub count: U16<LE>,
    pub nbt: NBTCompound<LittleEndian>,
    pub can_be_placed_on: SliceU32<StringUTF>,
    pub can_break: SliceU32<StringUTF>,
}

impl Binary for ItemStack {
    fn serialize<W: Write>(&self, buf: &mut W) {
        self.item_type.network_id.serialize(buf);

        if self.item_type.network_id.0 == 0 {
            return;
        }

        self.count.serialize(buf);
        self.item_type.metadata_value.serialize(buf);
        self.block_runtime_id.serialize(buf);

        self.can_be_placed_on.serialize(buf);
        self.can_break.serialize(buf);
    }

    fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
        Err(Error::new(ErrorKind::Other, ""))
    }
}

#[derive(Default, Debug)]
pub struct ItemType {
    pub network_id: VarI32,
    pub metadata_value: VarU32,
}

#[derive(Default, Debug, Binary)]
pub struct ItemEntry {
    pub name: VarString,
    pub runtime_id: I16<LE>,
    pub component_based: Bool,
}
