use std::io::{Read, Result, Write};

use binary::{
    binary::Binary,
    datatypes::{Bool, VarI32, VarI64, VarU32, VarU64, F32, I32, I64, U64, U8},
};
use binary_derive::Binary;
use byteorder::{BE, LE};
use bytes::Buf;
use commons::is_variant;
use nbt::{encoding::NetworkLittleEndian, NBTCompound};

use crate::data::{
    block::BlockEntry,
    education::EducationSharedResourceURI,
    entity::attribute::Attribute,
    entity::mob::{MobEffectOperation, MobEffectType},
    entity::teleport::{MovementMode, TeleportCause},
    event::{ActorEventType, LevelEventType},
    game::{
        ChatRestriction, EditorWorldType, ExperimentData, GamePublishSetting, GameRule, GameType,
        PlayerPermission, SpawnBiomeType,
    },
    generic::{InteractAction, PlayStatusType, TextType},
    item::{ItemEntry, ItemInstance},
    player::PlayerMoveSettings,
    resource_pack::{
        BehaviourPackInfo, PackURL, ResourcePackResponse, StackResourcePack, TexturePackInfo,
    },
    ByteSlice, Position, Rotation, Slice, SliceU16, SliceU32, UBlockPos, VarString, Velocity, UUID,
};

#[derive(Default, Debug, Binary)]
pub struct Login {
    pub protocol: I32<BE>,
    pub data: ByteSlice,
}

#[derive(Default, Debug, Binary)]
pub struct PlayStatus {
    pub status: PlayStatusType,
}

#[derive(Default, Debug, Binary)]
pub struct S2CHandshake {
    pub jwt: ByteSlice,
}

#[derive(Default, Debug, Binary)]
pub struct C2SHandshake;

#[derive(Default, Debug)]
pub struct Disconnect {
    pub hide_screen: Bool,
    pub message: Option<VarString>,
}

///
/// Special Serialization and Deserialization of Disconnect Packet exists because we want
/// to encode the message only if the screen is not meant to be hidden.
///
impl Binary for Disconnect {
    fn serialize<W: Write>(&self, buf: &mut W) {
        self.hide_screen.serialize(buf);

        if !self.hide_screen.0 {
            self.message.as_ref().unwrap().serialize(buf);
        }
    }

    fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
        let hide_screen = Bool::deserialize(buf)?;
        let mut message = None;

        if !hide_screen.0 {
            message = Some(VarString::deserialize(buf)?);
        }

        Ok(Self {
            hide_screen,
            message,
        })
    }
}

#[derive(Default, Debug, Binary)]
pub struct ResourcePacksInfo {
    pub texture_pack_required: Bool,
    pub has_scripts: Bool,
    pub forcing_server_packs: Bool,
    pub behaviour_packs: SliceU16<BehaviourPackInfo>,
    pub texture_packs: SliceU16<TexturePackInfo>,
    pub pack_urls: Slice<PackURL>,
}

#[derive(Default, Debug, Binary)]
pub struct ResourcePackStack {
    pub texture_pack_required: Bool,
    pub behaviour_packs: Slice<StackResourcePack>,
    pub texture_packs: Slice<StackResourcePack>,
    pub base_game_version: VarString,
    pub experiments: SliceU32<ExperimentData>,
    pub experiments_previously_toggled: Bool,
}

#[derive(Default, Debug, Binary)]
pub struct ResourcePackClientResponse {
    pub response: ResourcePackResponse,
    pub packs: SliceU16<VarString>,
}

#[derive(Default, Debug)]
pub struct Text {
    pub text_type: TextType,
    pub needs_translation: Bool,
    pub source_name: VarString,
    pub message: VarString,
    pub parameters: Slice<VarString>,
    pub xuid: VarString,
    pub platform_chat_id: VarString,
}

impl Binary for Text {
    fn serialize<W: Write>(&self, buf: &mut W) {
        self.text_type.serialize(buf);
        self.needs_translation.serialize(buf);

        if is_variant!(TextType, self.text_type, Chat, Whisper, Announcement) {
            self.source_name.serialize(buf);
            self.message.serialize(buf);
        } else if is_variant!(
            TextType,
            self.text_type,
            Raw,
            Tip,
            System,
            Object,
            ObjectWhisper,
            ObjectAnnouncement
        ) {
            self.message.serialize(buf);
        } else if is_variant!(TextType, self.text_type, Translation, Popup, JukeboxPopup) {
            self.message.serialize(buf);
            self.parameters.serialize(buf);
        }

        self.xuid.serialize(buf);
        self.platform_chat_id.serialize(buf);
    }

    fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
        let text_type = TextType::deserialize(buf)?;
        let needs_translation = Bool::deserialize(buf)?;
        let mut source_name = Default::default();
        let mut message = Default::default();
        let mut parameters = Default::default();

        if is_variant!(TextType, text_type, Chat, Whisper, Announcement) {
            source_name = Binary::deserialize(buf)?;
            message = Binary::deserialize(buf)?;
        } else if is_variant!(
            TextType,
            text_type,
            Raw,
            Tip,
            System,
            Object,
            ObjectWhisper,
            ObjectAnnouncement
        ) {
            message = Binary::deserialize(buf)?;
        } else if is_variant!(TextType, text_type, Translation, Popup, JukeboxPopup) {
            message = Binary::deserialize(buf)?;
            parameters = Binary::deserialize(buf)?;
        }

        let xuid = Binary::deserialize(buf)?;
        let platform_chat_id = Binary::deserialize(buf)?;

        Ok(Self {
            text_type,
            needs_translation,
            source_name,
            message,
            parameters,
            xuid,
            platform_chat_id,
        })
    }
}

#[derive(Default, Debug, Binary)]
pub struct SetTime {
    pub time: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct StartGame {
    pub entity_unique_id: VarI64,
    pub entity_runtime_id: VarU64,
    pub game_type: GameType,
    pub position: Position,
    pub pitch: F32<LE>,
    pub yaw: F32<LE>,
    pub world_seed: I64<LE>,
    pub spawn_biome_type: SpawnBiomeType,
    pub user_defined_biome_name: VarString,
    pub dimension: VarI32,
    pub generator: VarI32,
    pub world_game_type: GameType,
    pub difficulty: VarI32,
    pub world_spawn: UBlockPos,
    pub achievements_disabled: Bool,
    pub editor_world_type: EditorWorldType,
    pub created_in_editor: Bool,
    pub exported_from_editor: Bool,
    pub day_cycle_lock_time: VarI32,
    pub education_edition_offer: VarI32,
    pub education_features_enabled: Bool,
    pub education_product_id: VarString,
    pub rain_level: F32<LE>,
    pub lightning_level: F32<LE>,
    pub confirmed_platform_locked_content: Bool,
    pub multiplayer_game: Bool,
    pub lan_broadcast_enabled: Bool,
    pub xbl_broadcast_mode: GamePublishSetting,
    pub platform_broadcast_mode: GamePublishSetting,
    pub commands_enabled: Bool,
    pub texture_pack_required: Bool,
    pub game_rules: Slice<GameRule>,
    pub experiments: SliceU32<ExperimentData>,
    pub experiments_previously_toggled: Bool,
    pub bonus_chest_enabled: Bool,
    pub start_with_map_enabled: Bool,
    pub player_permission: PlayerPermission,
    pub server_chunk_tick_radius: I32<LE>,
    pub has_locked_behaviour_pack: Bool,
    pub has_locked_texture_pack: Bool,
    pub from_locked_world_template: Bool,
    pub msa_gamertags_only: Bool,
    pub from_world_template: Bool,
    pub world_template_settings_locked: Bool,
    pub only_spawn_v1_villagers: Bool,
    pub persona_disabled: Bool,
    pub custom_skins_disabled: Bool,
    pub emote_chat_muted: Bool,
    pub base_game_version: VarString,
    pub limited_world_width: I32<LE>,
    pub limited_world_depth: I32<LE>,
    pub new_nether: Bool,
    pub education_shared_uri: EducationSharedResourceURI,
    pub force_experimental_gameplay: Bool,
    pub chat_restriction_level: ChatRestriction,
    pub disable_player_interactions: Bool,
    pub level_id: VarString,
    pub world_name: VarString,
    pub template_content_identity: VarString,
    pub trial: Bool,
    pub player_move_settings: PlayerMoveSettings,
    pub time: I64<LE>,
    pub enchantment_seed: VarI32,
    pub blocks: Slice<BlockEntry>,
    pub items: Slice<ItemEntry>,
    pub multiplayer_correlation_id: VarString,
    pub server_authoritative_inventory: Bool,
    pub game_version: VarString,
    pub property_data: NBTCompound<NetworkLittleEndian>,
    pub server_blockstate_checksum: U64<LE>,
    pub world_template_id: UUID,
    pub clientside_generation: Bool,
    pub use_block_network_id_hashes: Bool,
    pub server_authoritative_sound: Bool,
}

#[derive(Default, Debug, Binary)]
pub struct AddPlayer {
    pub uuid: UUID,
    pub username: VarString,
    pub entity_runtime_id: VarU64,
    pub platform_chat_id: VarString,
    pub position: Position,
    pub velocity: Velocity,
    pub pitch: F32<LE>,
    pub yaw: F32<LE>,
    pub head_yaw: F32<LE>,
    pub held_item: ItemInstance,
    pub game_type: GameType,
    pub device_id: VarString,
    pub build_platform: I32<LE>,
}

#[derive(Default, Debug, Binary)]
pub struct TakeItemActor {
    pub item_runtime_id: VarU64,
    pub taker_runtime_id: VarU64,
}

#[derive(Default, Debug, Binary)]
pub struct MoveActorAbsolute {
    pub entity_runtime_id: VarU64,
    pub flags: U8,
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(Default, Debug, Binary)]
pub struct MovePlayer {
    pub entity_runtime_id: VarU64,
    pub position: Position,
    pub pitch: F32<LE>,
    pub yaw: F32<LE>,
    pub head_yaw: F32<LE>,
    pub mode: MovementMode,
    pub on_ground: Bool,
    pub ridden_runtime_id: VarU64,
    pub cause: TeleportCause,
    pub source_entity_type: I32<LE>,
    pub tick: VarU64,
}

#[derive(Default, Debug, Binary)]
pub struct PassengerJump {
    pub strength: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct UpdateBlock {
    pub position: UBlockPos,
    pub new_runtime_id: VarU32,
    pub flags: VarU32,
    pub layer: VarU32,
}

#[derive(Default, Debug, Binary)]
pub struct AddPainting {
    pub entity_unique_id: VarI64,
    pub entity_runtime_id: VarU64,
    pub position: Position,
    pub direction: VarI32,
    pub title: VarString,
}

#[derive(Default, Debug, Binary)]
pub struct TickSync {
    pub client_request_timestamp: I64<LE>,
    pub server_reception_timestamp: I64<LE>,
}

#[derive(Default, Debug, Binary)]
pub struct LevelEvent {
    pub event: LevelEventType,
    pub position: Position,
    pub data: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct BlockEvent {
    pub position: UBlockPos,
    pub event_type: VarI32,
    pub data: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct ActorEvent {
    pub entity_runtime_id: VarU64,
    pub event_type: ActorEventType,
    pub event_data: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct MobEffect {
    pub entity_runtime_id: VarU64,
    pub operation: MobEffectOperation,
    pub effect_type: MobEffectType,
    pub amplifier: VarI32,
    pub particles: Bool,
    pub duration: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct UpdateAttributes {
    pub entity_runtime_id: VarU64,
    pub attributes: Slice<Attribute>,
    pub tick: VarU64,
}

#[derive(Default, Debug, Binary)]
pub struct Interact {
    pub action_type: InteractAction,
    pub target_runtime_id: VarU64,
    pub position: Position,
}

#[derive(Default, Debug, Binary)]
pub struct BlockPickRequest {
    pub position: Position,
    pub add_block_nbt: Bool,
    pub hotbar_style: U8,
}

#[derive(Default, Debug, Binary)]
pub struct ActorPickRequest {
    pub entity_unique_id: I64<LE>,
    pub hotbar_slot: U8,
    pub with_data: Bool,
}

#[derive(Default, Debug, Binary)]
pub struct PlayerAction {
    pub entity_runtime_id: VarU64,
    pub action_type: VarI32,
    pub block_pos: UBlockPos,
    pub result_pos: UBlockPos,
    pub block_face: VarI32,
}

#[derive(Default, Debug, Binary)]
pub struct HurtArmor {
    pub cause: VarI32,
    pub damage: VarI32,
}
