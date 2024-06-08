use crate::result::Result;
use apsak_consensus_core::hashing::sighash::{calc_schnorr_signature_hash, SigHashReusedValues};
use apsak_consensus_core::hashing::sighash_type::SIG_HASH_ALL;
use apsak_consensus_core::tx;

pub fn script_hashes(mut mutable_tx: tx::SignableTransaction) -> Result<Vec<apsak_hashes::Hash>> {
    let mut list = vec![];
    for i in 0..mutable_tx.tx.inputs.len() {
        mutable_tx.tx.inputs[i].sig_op_count = 1;
    }

    let mut reused_values = SigHashReusedValues::new();
    for i in 0..mutable_tx.tx.inputs.len() {
        let sig_hash = calc_schnorr_signature_hash(&mutable_tx.as_verifiable(), i, SIG_HASH_ALL, &mut reused_values);
        list.push(sig_hash);
    }
    Ok(list)
}
