use binary_derive::Binary;

#[derive(Debug, Default, Binary)]
#[data(datatype = "U8")]
pub enum NPCRequestType {
    SetAction,
    ExecuteCommandAction,
    ExecuteClosingCommands,
    SetName,
    SetSkin,
    SetInteractionText,
    ExecuteOpeningCommands,
    #[default]
    Invalid,
}
