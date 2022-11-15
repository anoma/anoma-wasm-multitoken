//! Helpers for reading from storage

use eyre::{eyre, Context, Result};
use namada_vp_prelude::{storage::Key, token::Amount, BorshDeserialize, Ctx, VpEnv};

pub(crate) fn amount(ctx: &Ctx, key: &Key) -> Result<(Amount, Amount)> {
    let pre = if let Some(bytes) = ctx.read_bytes_pre(key).unwrap() {
        Amount::try_from_slice(&bytes[..]).wrap_err("couldn't deserialize pre to Amount")?
    } else {
        Amount::from(0)
    };
    let post = if let Some(bytes) = ctx.read_bytes_post(key).unwrap() {
        Amount::try_from_slice(&bytes[..]).wrap_err("couldn't deserialize post to Amount")?
    } else {
        return Err(eyre!("read_bytes_post didn't read anything"));
    };
    Ok((pre, post))
}
