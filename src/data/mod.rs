use std::io::{Read, Result, Write};

use binary::{
    binary::Binary,
    datatypes::{VarI32, VarU32, F32, I16, U16, U32, U8},
    prefixed::{Array, Str},
};
use binary_derive::Binary;
use byteorder::LE;
use bytes::Buf;
use uuid::Uuid;

pub mod block;
pub mod education;
pub mod entity;
pub mod event;
pub mod game;
pub mod generic;
pub mod inventory;
pub mod item;
pub mod player;
pub mod resource_pack;

/// This is an alias for a String whose prefix is encoded in the format of
/// an Unsigned VarInt.
pub type VarString = Str<VarU32>;

/// This is an alias for a String whose prefix is encoded in the format of signed
/// 16-bit little endian integer.
pub type StringUTF = Str<I16<LE>>;

/// This is an alias for an array of bytes whose prefix is encoded in the format
/// of Unsigned VarInt.
pub type ByteSlice = Slice<U8>;

/// This is an alias for an array whose prefix is encoded in the format of Unsigned
/// VarInt.
pub type Slice<B> = Array<B, VarU32>;

/// This is an alias for an array whose prefix is encoded in the format of unsigned
/// 32 bit integer.
pub type SliceU32<B> = Array<B, U32<LE>>;

/// This is an alias for an array whose prefix is encoded in the format of unsigned
/// 16 bit integer.
pub type SliceU16<B> = Array<B, U16<LE>>;

/// A position of an entity in a Minecraft World is represented by three
/// coordinates x, y, and z.
#[derive(Binary, Debug, Clone, Default)]
pub struct Position {
    pub x: F32<LE>,
    pub y: F32<LE>,
    pub z: F32<LE>,
}

/// Rotation specifies the rotation of the entity with respect to the three axis:
/// x, y, z axis. This exists due to the reason that specific entities like arrows do not have
/// yaw/pitch.
#[derive(Debug, Clone, Default)]
pub struct Rotation {
    pub x: F32<LE>,
    pub y: F32<LE>,
    pub z: F32<LE>,
}

const ROT: f32 = 360.0 / 256.0;

impl Binary for Rotation {
    fn serialize<W: Write>(&self, buf: &mut W) {
        let x = (self.x.0 / ROT) as u8;
        let y = (self.y.0 / ROT) as u8;
        let z = (self.z.0 / ROT) as u8;

        U8::new(x).serialize(buf);
        U8::new(y).serialize(buf);
        U8::new(z).serialize(buf);
    }

    fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
        let x = (U8::deserialize(buf)?.0 as f32) * ROT;
        let y = (U8::deserialize(buf)?.0 as f32) * ROT;
        let z = (U8::deserialize(buf)?.0 as f32) * ROT;

        Ok(Self {
            x: F32::new(x),
            y: F32::new(y),
            z: F32::new(z),
        })
    }
}

/// BlockPos is the position of the block. It is composed of three VarI32s'.
#[derive(Default, Debug, Clone, Binary)]
pub struct BlockPos {
    pub x: VarI32,
    pub y: VarI32,
    pub z: VarI32,
}

/// UBlockPos is the position of the block. It is composed of two VarI32s'
/// and a VarU32.
#[derive(Default, Debug, Clone, Binary)]
pub struct UBlockPos {
    pub x: VarI32,
    pub y: VarU32,
    pub z: VarI32,
}

/// A velocity of an entity in a Minecraft World along the three axis: x, y, and z.
#[derive(Binary, Debug, Clone, Default)]
pub struct Velocity {
    pub x: F32<LE>,
    pub y: F32<LE>,
    pub z: F32<LE>,
}

/// Unique ID (UUID) may be the unique ID of any Minecraft Object such as Player, World Templates, etc.
#[derive(Default, Debug)]
pub struct UUID(pub Uuid);

impl Binary for UUID {
    fn serialize<W: Write>(&self, buf: &mut W) {
        let bytes = self.0.as_bytes();
        buf.write_all(bytes).unwrap();
    }

    fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
        let mut bytes = [0u8; 16];
        buf.read_exact(&mut bytes)?;

        let uuid = Uuid::from_bytes(bytes);
        Ok(Self(uuid))
    }
}
