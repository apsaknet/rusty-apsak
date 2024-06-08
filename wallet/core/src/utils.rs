//!
//! apsaK value formatting and parsing utilities.
//!

use crate::result::Result;
use apsak_addresses::Address;
use apsak_consensus_core::constants::*;
use apsak_consensus_core::network::NetworkType;
use separator::{separated_float, separated_int, separated_uint_with_output, Separatable};
use workflow_log::style;

pub fn try_apsak_str_to_ipmos<S: Into<String>>(s: S) -> Result<Option<u64>> {
    let s: String = s.into();
    let amount = s.trim();
    if amount.is_empty() {
        return Ok(None);
    }

    Ok(Some(str_to_ipmos(amount)?))
}

pub fn try_apsak_str_to_ipmos_i64<S: Into<String>>(s: S) -> Result<Option<i64>> {
    let s: String = s.into();
    let amount = s.trim();
    if amount.is_empty() {
        return Ok(None);
    }

    let amount = amount.parse::<f64>()? * IPMOS_PER_APSAK as f64;
    Ok(Some(amount as i64))
}

#[inline]
pub fn ipmos_to_apsak(ipmos: u64) -> f64 {
    ipmos as f64 / IPMOS_PER_APSAK as f64
}

#[inline]
pub fn apsak_to_ipmos(apsak: f64) -> u64 {
    (apsak * IPMOS_PER_APSAK as f64) as u64
}

#[inline]
pub fn ipmos_to_apsak_string(ipmos: u64) -> String {
    ipmos_to_apsak(ipmos).separated_string()
}

#[inline]
pub fn ipmos_to_apsak_string_with_trailing_zeroes(ipmos: u64) -> String {
    separated_float!(format!("{:.8}", ipmos_to_apsak(ipmos)))
}

pub fn apsak_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "SAK",
        NetworkType::Testnet => "TSAK",
        NetworkType::Simnet => "SSAK",
        NetworkType::Devnet => "DSAK",
    }
}

#[inline]
pub fn ipmos_to_apsak_string_with_suffix(ipmos: u64, network_type: &NetworkType) -> String {
    let sak = ipmos_to_apsak_string(ipmos);
    let suffix = apsak_suffix(network_type);
    format!("{sak} {suffix}")
}

#[inline]
pub fn ipmos_to_apsak_string_with_trailing_zeroes_and_suffix(ipmos: u64, network_type: &NetworkType) -> String {
    let sak = ipmos_to_apsak_string_with_trailing_zeroes(ipmos);
    let suffix = apsak_suffix(network_type);
    format!("{sak} {suffix}")
}

pub fn format_address_colors(address: &Address, range: Option<usize>) -> String {
    let address = address.to_string();

    let parts = address.split(':').collect::<Vec<&str>>();
    let prefix = style(parts[0]).dim();
    let payload = parts[1];
    let range = range.unwrap_or(6);
    let start = range;
    let finish = payload.len() - range;

    let left = &payload[0..start];
    let center = style(&payload[start..finish]).dim();
    let right = &payload[finish..];

    format!("{prefix}:{left}:{center}:{right}")
}

fn str_to_ipmos(amount: &str) -> Result<u64> {
    let Some(dot_idx) = amount.find('.') else {
        return Ok(amount.parse::<u64>()? * IPMOS_PER_APSAK);
    };
    let integer = amount[..dot_idx].parse::<u64>()? * IPMOS_PER_APSAK;
    let decimal = &amount[dot_idx + 1..];
    let decimal_len = decimal.len();
    let decimal = if decimal_len == 0 {
        0
    } else if decimal_len <= 8 {
        decimal.parse::<u64>()? * 10u64.pow(8 - decimal_len as u32)
    } else {
        // TODO - discuss how to handle values longer than 8 decimal places
        // (reject, truncate, ceil(), etc.)
        decimal[..8].parse::<u64>()?
    };
    Ok(integer + decimal)
}
