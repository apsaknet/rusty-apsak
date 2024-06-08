use crate::pb::apsakd_message::Payload as ApsakdMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum ApsakdMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&ApsakdMessagePayload> for ApsakdMessagePayloadType {
    fn from(payload: &ApsakdMessagePayload) -> Self {
        match payload {
            ApsakdMessagePayload::Addresses(_) => ApsakdMessagePayloadType::Addresses,
            ApsakdMessagePayload::Block(_) => ApsakdMessagePayloadType::Block,
            ApsakdMessagePayload::Transaction(_) => ApsakdMessagePayloadType::Transaction,
            ApsakdMessagePayload::BlockLocator(_) => ApsakdMessagePayloadType::BlockLocator,
            ApsakdMessagePayload::RequestAddresses(_) => ApsakdMessagePayloadType::RequestAddresses,
            ApsakdMessagePayload::RequestRelayBlocks(_) => ApsakdMessagePayloadType::RequestRelayBlocks,
            ApsakdMessagePayload::RequestTransactions(_) => ApsakdMessagePayloadType::RequestTransactions,
            ApsakdMessagePayload::IbdBlock(_) => ApsakdMessagePayloadType::IbdBlock,
            ApsakdMessagePayload::InvRelayBlock(_) => ApsakdMessagePayloadType::InvRelayBlock,
            ApsakdMessagePayload::InvTransactions(_) => ApsakdMessagePayloadType::InvTransactions,
            ApsakdMessagePayload::Ping(_) => ApsakdMessagePayloadType::Ping,
            ApsakdMessagePayload::Pong(_) => ApsakdMessagePayloadType::Pong,
            ApsakdMessagePayload::Verack(_) => ApsakdMessagePayloadType::Verack,
            ApsakdMessagePayload::Version(_) => ApsakdMessagePayloadType::Version,
            ApsakdMessagePayload::TransactionNotFound(_) => ApsakdMessagePayloadType::TransactionNotFound,
            ApsakdMessagePayload::Reject(_) => ApsakdMessagePayloadType::Reject,
            ApsakdMessagePayload::PruningPointUtxoSetChunk(_) => ApsakdMessagePayloadType::PruningPointUtxoSetChunk,
            ApsakdMessagePayload::RequestIbdBlocks(_) => ApsakdMessagePayloadType::RequestIbdBlocks,
            ApsakdMessagePayload::UnexpectedPruningPoint(_) => ApsakdMessagePayloadType::UnexpectedPruningPoint,
            ApsakdMessagePayload::IbdBlockLocator(_) => ApsakdMessagePayloadType::IbdBlockLocator,
            ApsakdMessagePayload::IbdBlockLocatorHighestHash(_) => ApsakdMessagePayloadType::IbdBlockLocatorHighestHash,
            ApsakdMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                ApsakdMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            ApsakdMessagePayload::DonePruningPointUtxoSetChunks(_) => ApsakdMessagePayloadType::DonePruningPointUtxoSetChunks,
            ApsakdMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                ApsakdMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            ApsakdMessagePayload::BlockWithTrustedData(_) => ApsakdMessagePayloadType::BlockWithTrustedData,
            ApsakdMessagePayload::DoneBlocksWithTrustedData(_) => ApsakdMessagePayloadType::DoneBlocksWithTrustedData,
            ApsakdMessagePayload::RequestPruningPointAndItsAnticone(_) => ApsakdMessagePayloadType::RequestPruningPointAndItsAnticone,
            ApsakdMessagePayload::BlockHeaders(_) => ApsakdMessagePayloadType::BlockHeaders,
            ApsakdMessagePayload::RequestNextHeaders(_) => ApsakdMessagePayloadType::RequestNextHeaders,
            ApsakdMessagePayload::DoneHeaders(_) => ApsakdMessagePayloadType::DoneHeaders,
            ApsakdMessagePayload::RequestPruningPointUtxoSet(_) => ApsakdMessagePayloadType::RequestPruningPointUtxoSet,
            ApsakdMessagePayload::RequestHeaders(_) => ApsakdMessagePayloadType::RequestHeaders,
            ApsakdMessagePayload::RequestBlockLocator(_) => ApsakdMessagePayloadType::RequestBlockLocator,
            ApsakdMessagePayload::PruningPoints(_) => ApsakdMessagePayloadType::PruningPoints,
            ApsakdMessagePayload::RequestPruningPointProof(_) => ApsakdMessagePayloadType::RequestPruningPointProof,
            ApsakdMessagePayload::PruningPointProof(_) => ApsakdMessagePayloadType::PruningPointProof,
            ApsakdMessagePayload::Ready(_) => ApsakdMessagePayloadType::Ready,
            ApsakdMessagePayload::BlockWithTrustedDataV4(_) => ApsakdMessagePayloadType::BlockWithTrustedDataV4,
            ApsakdMessagePayload::TrustedData(_) => ApsakdMessagePayloadType::TrustedData,
            ApsakdMessagePayload::RequestIbdChainBlockLocator(_) => ApsakdMessagePayloadType::RequestIbdChainBlockLocator,
            ApsakdMessagePayload::IbdChainBlockLocator(_) => ApsakdMessagePayloadType::IbdChainBlockLocator,
            ApsakdMessagePayload::RequestAntipast(_) => ApsakdMessagePayloadType::RequestAntipast,
            ApsakdMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                ApsakdMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
