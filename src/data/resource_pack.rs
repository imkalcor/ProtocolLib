use binary::datatypes::{Bool, U64};
use binary_derive::Binary;
use byteorder::LE;

use super::VarString;

#[derive(Default, Debug, Binary)]
#[data(datatype = "U8")]
pub enum ResourcePackResponse {
    #[variant(tag = 1)]
    Refused,
    SendPacks,
    AllPacksDownloaded,
    Completed,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
pub struct BehaviourPackInfo {
    pub uuid: VarString,
    pub version: VarString,
    pub size: U64<LE>,
    pub content_key: VarString,
    pub subpack_name: VarString,
    pub content_identity: VarString,
    pub has_scripts: Bool,
}

#[derive(Default, Debug, Binary)]
pub struct TexturePackInfo {
    pub uuid: VarString,
    pub version: VarString,
    pub size: U64<LE>,
    pub content_key: VarString,
    pub subpack_name: VarString,
    pub content_identity: VarString,
    pub has_scripts: Bool,
    pub rtx_enabled: Bool,
}

#[derive(Default, Debug, Binary)]
pub struct StackResourcePack {
    pub uuid: VarString,
    pub version: VarString,
    pub subpack_name: VarString,
}

#[derive(Default, Debug, Binary)]
pub struct PackURL {
    pub uuid_version: VarString,
    pub url: VarString,
}
