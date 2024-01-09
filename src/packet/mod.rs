#![allow(non_snake_case)]

use crate::packet::header::PacketHeader;
use binary::binary::Binary;
use bytes::Buf;
use std::io::{Error, ErrorKind, Read, Result, Write};

use self::{
    play::{
        ActorEvent, AddPainting, AddPlayer, BlockEvent, C2SHandshake, Disconnect, LevelEvent,
        Login, MobEffect, MoveActorAbsolute, MovePlayer, PassengerJump, PlayStatus,
        ResourcePackClientResponse, ResourcePackStack, ResourcePacksInfo, S2CHandshake, SetTime,
        StartGame, TakeItemActor, Text, TickSync, UpdateAttributes, UpdateBlock,
    },
    prelogin::{NetworkSettings, RequestNetworkSettings},
};

pub mod header;
pub mod play;
pub mod prelogin;

#[macro_export]
macro_rules! build_packet {
    (
            $(
                $packet:ident($struct:ident) = $id:expr
            ),*
    ) => {
        /// Minecraft Packet represents a packet that may be sent over a Minecraft Connection.
        #[derive(Debug)]
        pub enum Packet {
            $(
                $packet($struct)
            ),*
        }

        /// PacketID represents the Minecraft Packet ID.
        #[derive(Debug, Hash, Eq, PartialEq)]
        #[repr(u32)]
        pub enum PacketID {
            $(
                $packet = $id
            ),*
        }

        impl Packet {
            /// Returns the ID of the Minecraft Packet.
            pub fn id(&self) -> PacketID {
                match self {
                    $(
                        Self::$packet { .. } => PacketID::$packet,
                    )*
                }
            }
        }

        impl Binary for Packet {
            fn serialize<W: Write>(&self, buf: &mut W) {
                let header = PacketHeader::new(self.id() as u32);
                header.serialize(buf);

                match self {
                    $(
                        Self::$packet($struct) => $struct.serialize(buf),
                    )*
                }
            }

            fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
                let header = PacketHeader::deserialize(buf)?;
                let id = header.id();

                match id {
                    $(
                        $id => {
                            let inner = $struct::deserialize(buf)?;
                            Ok(Self::$packet(inner))
                        }
                    )*
                    _ => Err(Error::new(ErrorKind::Other, format!("Packet ID {} not found!", id))),
                }
            }
        }
    }
}

build_packet!(
    Login(Login) = 0x01,
    PlayStatus(PlayStatus) = 0x02,
    S2CHandshake(S2CHandshake) = 0x03,
    C2SHandshake(C2SHandshake) = 0x04,
    Disconnect(Disconnect) = 0x05,
    ResourcePacksInfo(ResourcePacksInfo) = 0x06,
    ResourcePackStack(ResourcePackStack) = 0x07,
    ResourcePackClientResponse(ResourcePackClientResponse) = 0x08,
    Text(Text) = 0x09,
    SetTime(SetTime) = 0x0a,
    StartGame(StartGame) = 0x0b,
    AddPlayer(AddPlayer) = 0x0c,
    TakeItemActor(TakeItemActor) = 0x11,
    MoveActorAbsolute(MoveActorAbsolute) = 0x12,
    MovePlayer(MovePlayer) = 0x13,
    PassengerJump(PassengerJump) = 0x14,
    UpdateBlock(UpdateBlock) = 0x15,
    AddPainting(AddPainting) = 0x16,
    TickSync(TickSync) = 0x17,
    LevelEvent(LevelEvent) = 0x19,
    BlockEvent(BlockEvent) = 0x1a,
    ActorEvent(ActorEvent) = 0x1b,
    MobEffect(MobEffect) = 0x1c,
    UpdateAttributes(UpdateAttributes) = 0x1d,
    RequestNetworkSettings(RequestNetworkSettings) = 0xc1,
    NetworkSettings(NetworkSettings) = 0x8f
);
