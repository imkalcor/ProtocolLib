use std::io::{Read, Result, Write};

use binary::binary::Binary;
use binary::datatypes::VarU32;
use bytes::Buf;

const PID_MASK: u32 = 0x3FF;
const SUBCLIENT_ID_MASK: u32 = 0x03;
const SENDER_SUBCLIENT_ID_SHIFT: u32 = 10;
const TARGET_SUBCLIENT_ID_SHIFT: u32 = 12;

/// Packet Header is the header of the packet. It exists out of a single VarU32 which is composed of the packet ID
/// and a sender and a target sub client ID. These IDs are used for split screen functionality.
#[derive(Debug, Clone, Default)]
pub struct PacketHeader {
    id: u32,
    sender_subclient_id: u8,
    target_subclient_id: u8,
}

impl PacketHeader {
    /// Creates and returns a new Packet Header.
    pub fn new(id: u32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Returns the ID associated with the Packet Header.
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Binary for PacketHeader {
    fn serialize<W: Write>(&self, buf: &mut W) {
        let header = self.id
            | (self.sender_subclient_id as u32) << SENDER_SUBCLIENT_ID_SHIFT
            | (self.target_subclient_id as u32) << TARGET_SUBCLIENT_ID_SHIFT;
        VarU32::new(header).serialize(buf);
    }

    fn deserialize<R: Read + Buf>(buf: &mut R) -> Result<Self> {
        let val = VarU32::deserialize(buf)?.0;

        let id = val & PID_MASK;
        let sender_subclient_id = ((val >> SENDER_SUBCLIENT_ID_SHIFT) & SUBCLIENT_ID_MASK) as u8;
        let target_subclient_id = ((val >> TARGET_SUBCLIENT_ID_SHIFT) & SUBCLIENT_ID_MASK) as u8;

        Ok(Self {
            id,
            sender_subclient_id,
            target_subclient_id,
        })
    }
}
