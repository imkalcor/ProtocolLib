use binary::datatypes::{Bool, F32, I32, U16, U8};
use binary_derive::Binary;
use byteorder::{BE, LE};

/// RequestNetworkSettings is sent by the client to request network settings, such as compression,
/// from the server.
#[derive(Default, Debug, Binary)]
pub struct RequestNetworkSettings {
    /// ClientProtocol is the protocol version of the player. The player is disconnected if
    /// the protocol is incompatible with the protocol of the server.
    pub client_protocol: I32<BE>,
}

/// NetworkSettings is sent by the server to update a variety of network settings. These settings modify the
/// way packets are sent over the network stack.
#[derive(Default, Debug, Binary)]
pub struct NetworkSettings {
    /// CompressionThreshold is the minimum size of a packet that is compressed when sent. If the size of a
    /// packet is under this value, it is not compressed. When set to 0, all packets will be left uncompressed.
    pub compression_threshold: U16<LE>,

    /// CompressionAlgorithm is the algorithm that is used to compress packets.
    pub compression_alogrithm: U16<LE>,

    /// This regulates whether the client should throttle players when exceeding of the threshold. Players
    /// outside threshold will not be ticked, improving performance on low-end devices.
    pub client_throttle: Bool,

    /// It is the threshold for client throttling. If the number of players exceeds this value, the
    /// client will throttle players.
    pub client_throttle_threshold: U8,

    /// This is the scalar for client throttling. The scalar is the amount of players that are ticked
    /// when throttling is enabled.
    pub client_throttle_scalar: F32<LE>,
}
