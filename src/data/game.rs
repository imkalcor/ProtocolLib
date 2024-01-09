use binary::datatypes::{Bool, F32, U32};
use binary_derive::Binary;
use byteorder::LE;

use super::VarString;

#[derive(Default, Debug, Binary)]
pub struct ExperimentData {
    pub name: VarString,
    pub enabled: Bool,
}

#[derive(Default, Debug, Binary)]
pub struct GameRule {
    pub name: VarString,
    pub can_be_modified: Bool,
    pub data: GameRuleData,
}

#[derive(Default, Debug, Binary)]

pub enum GameRuleData {
    Boolean(Bool),
    Integer(U32<LE>),
    Float(F32<LE>),
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "VarI32")]
pub enum GameType {
    Survival,
    Creative,
    Adventure,
    SurvivalSpectator,
    CreativeSpectator,
    #[default]
    Fallback,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "I16")]
pub enum SpawnBiomeType {
    #[default]
    Default,
    UserDefined,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "VarI32")]
pub enum EditorWorldType {
    NotEditor,
    Project,
    TestLevel,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "VarI32")]
pub enum GamePublishSetting {
    NoMultiplayer,
    InviteOnly,
    FriendsOnly,
    FriendsOfFriends,
    Public,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "VarI32")]
pub enum PlayerPermission {
    Visitor,
    Member,
    Operator,
    Custom,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "U8")]
pub enum ChatRestriction {
    None,
    Dropped,
    Disabled,
    #[default]
    Invalid,
}
