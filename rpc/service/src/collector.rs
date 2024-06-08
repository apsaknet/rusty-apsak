use crate::converter::{consensus::ConsensusConverter, index::IndexConverter};
use apsak_notify::collector::CollectorFrom;

pub(crate) type CollectorFromConsensus = CollectorFrom<ConsensusConverter>;

pub(crate) type CollectorFromIndex = CollectorFrom<IndexConverter>;
