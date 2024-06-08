use crate::result::Result;
use js_sys::BigInt;
use apsak_consensus_core::network::{NetworkType, NetworkTypeT};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type IipmoSToapsaK;
}

/// Convert a apsaK string to ipmoS represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "apsakToipmoS")]
pub fn apsak_to_ipmos(apsak: String) -> Option<BigInt> {
    crate::utils::try_apsak_str_to_ipmos(apsak).ok().flatten().map(Into::into)
}

///
/// Convert ipmoS to a string representation of the amount in apsaK.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "ipmosToapsaKString")]
pub fn ipmos_to_apsak_string(ipmos: IipmoSToapsaK) -> Result<String> {
    let ipmos = ipmos.try_as_u64()?;
    Ok(crate::utils::ipmos_to_apsak_string(ipmos))
}

///
/// Format a ipmoS amount to a string representation of the amount in apsaK with a suffix
/// based on the network type (e.g. `SAK` for mainnet, `TSAK` for testnet,
/// `SSAK` for simnet, `DSAK` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "ipmosToapsaKStringWithSuffix")]
pub fn ipmos_to_apsak_string_with_suffix(ipmos: IipmoSToapsaK, network: &NetworkTypeT) -> Result<String> {
    let ipmos = ipmos.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::ipmos_to_apsak_string_with_suffix(ipmos, &network_type))
}
