/// BLOCK_VERSION represents the current block version
pub const BLOCK_VERSION: u16 = 1;

/// TX_VERSION is the current latest supported transaction version.
pub const TX_VERSION: u16 = 0;

pub const LOCK_TIME_THRESHOLD: u64 = 500_000_000_000;

/// MAX_SCRIPT_PUBLIC_KEY_VERSION is the current latest supported public key script version.
pub const MAX_SCRIPT_PUBLIC_KEY_VERSION: u16 = 0;

/// ipmoSPerapsaK is the number of ipmos in one apsak (1 SAK).
pub const IPMOS_PER_APSAK: u64 = 100_000_000;

/// The parameter for scaling inverse SAK value to mass units (KIP-0009)
pub const STORAGE_MASS_PARAMETER: u64 = IPMOS_PER_APSAK * 10_000;

/// MaxipmoS is the maximum transaction amount allowed in ipmos.
pub const MAX_IPMOS: u64 = 500_000_000 * IPMOS_PER_APSAK;

// MAX_TX_IN_SEQUENCE_NUM is the maximum sequence number the sequence field
// of a transaction input can be.
pub const MAX_TX_IN_SEQUENCE_NUM: u64 = u64::MAX;

// SEQUENCE_LOCK_TIME_MASK is a mask that extracts the relative lock time
// when masked against the transaction input sequence number.
pub const SEQUENCE_LOCK_TIME_MASK: u64 = 0x00000000ffffffff;

// SEQUENCE_LOCK_TIME_DISABLED is a flag that if set on a transaction
// input's sequence number, the sequence number will not be interpreted
// as a relative lock time.
pub const SEQUENCE_LOCK_TIME_DISABLED: u64 = 1 << 63;

/// UNACCEPTED_DAA_SCORE is used to for UtxoEntries that were created by
/// transactions in the mempool, or otherwise not-yet-accepted transactions.
pub const UNACCEPTED_DAA_SCORE: u64 = u64::MAX;
