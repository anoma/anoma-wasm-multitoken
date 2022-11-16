use eyre::Result;
use namada_tx_prelude::{storage::Key, token::Amount, Ctx, StorageWrite};

use crate::read;

/// Reads the `Amount` from key, applies update then writes it back
pub fn amount(ctx: &mut Ctx, key: &Key, update: impl Fn(&mut Amount)) -> Result<Amount> {
    let mut amount = read::amount(ctx, key)?;
    update(&mut amount);
    ctx.write(key, amount).unwrap();
    Ok(amount)
}
