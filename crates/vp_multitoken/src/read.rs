//! Helpers for reading from storage

use anoma_vp_prelude::{read_post, read_pre, token::Amount};
use eyre::{eyre, Result};

/// TODO: this should error if there is a value stored at key, but it isn't an Amount
pub fn amount(key: &str) -> Result<(Amount, Amount)> {
    let pre = match read_pre(key) {
        Some(amount) => amount,
        None => Amount::from(0),
    };
    let post = match read_post(key) {
        Some(amount) => amount,
        None => return Err(eyre!("no post amount found")),
    };
    Ok((pre, post))
}
