use eyre::Result;
use namada_tx_prelude::{token::Amount, write, Key};

use crate::read;

/// Reads the `Amount` from key, applies update then writes it back
pub fn amount(key: &Key, update: impl Fn(&mut Amount)) -> Result<Amount> {
    let key = key.to_string();
    let mut amount = read::amount(&key)?;
    update(&mut amount);
    write(&key, amount);
    Ok(amount)
}
