use binary_derive::Binary;

#[derive(Debug, Default, Binary)]
#[data(datatype = "I32BE")]
pub enum PlayStatusType {
    Success,
    OutdatedClient,
    OutdatedServer,
    PlayerSpawn,
    InvalidTenant,
    EduToVanillaMismatch,
    VanillaToEduMismatch,
    ServerIsFull,
    EditorToVanillaMismatch,
    VanillaToEditorMismatch,
    #[default]
    Invalid,
}

#[derive(Debug, Default, Binary)]
#[data(datatype = "U8")]
pub enum TextType {
    Raw,
    Chat,
    Translation,
    Popup,
    JukeboxPopup,
    Tip,
    System,
    Whisper,
    Announcement,
    ObjectWhisper,
    Object,
    ObjectAnnouncement,
    #[default]
    Invalid,
}

#[derive(Default, Debug, Binary)]
#[data(datatype = "U8")]
pub enum InteractAction {
    #[variant(tag = 3)]
    LeaveVehicle,
    MouseOverEntity,
    NPCOpen,
    OpenInventory,
    #[default]
    Invalid,
}
