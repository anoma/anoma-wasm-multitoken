//! Helpers for reading from storage

use anoma_vp_prelude::{read_bytes_post, read_bytes_pre, token::Amount, BorshDeserialize};
use eyre::{eyre, Context, Result};

pub(crate) fn amount(key: &str) -> Result<(Amount, Amount)> {
    let pre = if let Some(bytes) = read_bytes_pre(key) {
        Amount::try_from_slice(&bytes[..]).wrap_err("couldn't deserialize pre to Amount")?
    } else {
        Amount::from(0)
    };
    let post = if let Some(bytes) = read_bytes_post(key) {
        Amount::try_from_slice(&bytes[..]).wrap_err("couldn't deserialize post to Amount")?
    } else {
        return Err(eyre!("read_bytes_post didn't read anything"));
    };
    Ok((pre, post))
}
