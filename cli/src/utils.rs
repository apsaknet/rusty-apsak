use crate::error::Error;
use crate::result::Result;
use apsak_consensus_core::constants::IPMOS_PER_APSAK;
use std::fmt::Display;

pub fn try_parse_required_nonzero_apsak_as_ipmos_u64<S: ToString + Display>(apsak_amount: Option<S>) -> Result<u64> {
    if let Some(apsak_amount) = apsak_amount {
        let ipmos_amount = apsak_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied apsaK amount is not valid: '{apsak_amount}'")))?
            * IPMOS_PER_APSAK as f64;
        if ipmos_amount < 0.0 {
            Err(Error::custom("Supplied apsaK amount is not valid: '{apsak_amount}'"))
        } else {
            let ipmos_amount = ipmos_amount as u64;
            if ipmos_amount == 0 {
                Err(Error::custom("Supplied required apsak amount must not be a zero: '{apsak_amount}'"))
            } else {
                Ok(ipmos_amount)
            }
        }
    } else {
        Err(Error::custom("Missing apsaK amount"))
    }
}

pub fn try_parse_required_apsak_as_ipmos_u64<S: ToString + Display>(apsak_amount: Option<S>) -> Result<u64> {
    if let Some(apsak_amount) = apsak_amount {
        let ipmos_amount = apsak_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied apsaK amount is not valid: '{apsak_amount}'")))?
            * IPMOS_PER_APSAK as f64;
        if ipmos_amount < 0.0 {
            Err(Error::custom("Supplied apsaK amount is not valid: '{apsak_amount}'"))
        } else {
            Ok(ipmos_amount as u64)
        }
    } else {
        Err(Error::custom("Missing apsaK amount"))
    }
}

pub fn try_parse_optional_apsak_as_ipmos_i64<S: ToString + Display>(apsak_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(apsak_amount) = apsak_amount {
        let ipmos_amount = apsak_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied apsaK amount is not valid: '{apsak_amount}'")))?
            * IPMOS_PER_APSAK as f64;
        if ipmos_amount < 0.0 {
            Err(Error::custom("Supplied apsaK amount is not valid: '{apsak_amount}'"))
        } else {
            Ok(Some(ipmos_amount as i64))
        }
    } else {
        Ok(None)
    }
}
