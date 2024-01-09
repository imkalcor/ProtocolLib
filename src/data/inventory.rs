use binary::datatypes::{VarI32, VarU32, VarU64, U8};
use binary_derive::Binary;

use super::ByteSlice;

#[derive(Debug, Default, Binary)]
#[data(datatype = "VarU32")]
pub enum InventorySourceType {
    Invalid = -1,
    Container,
    Global,
    WorldInteraction,
    Creative,
    UntrackedInteractionUI,
    #[default]
    #[variant(tag = 99999)]
    NonImplemented,
}

#[derive(Debug, Default, Binary)]
#[data(datatype = "VarU32")]
pub enum InventorySourceFlag {
    DropItem,
    PickupItem,
    #[default]
    None,
}

#[derive(Debug, Binary, Default)]
pub struct InventorySource {
    pub source_type: InventorySourceType,
    pub window_id: VarI32,
    pub flags: InventorySourceFlag,
}

#[derive(Debug, Binary, Default)]
pub struct InventoryAction {
    pub source: InventorySource,
    pub slot: VarU32,
}

#[derive(Debug, Binary, Default)]
pub struct LegacyItemSlot {
    pub container: U8,
    pub slots: ByteSlice,
}

#[derive(Binary, Debug, Default)]
#[data(datatype = "VarU32")]
#[repr(u32)]
pub enum InventoryTransactionData {
    Normal = 0,
    Mismatch,
    UseItem {
        action_type: VarU32,
        blockface: VarI32,
        hotbar_slot: VarI32,
    },
    UseItemOnEntity {
        target_runtime_id: VarU64,
        action_type: VarU32,
        hotbar_slot: VarI32,
    },
    ReleaseItem {
        action_type: VarU32,
        hotbar_slot: VarI32,
        item: VarI32,
    },
    #[default]
    Invalid,
}
