//! Helpers for reading from storage

use anoma_tx_prelude::{read, token::Amount};

/// Returns the stored Amount, or 0 if not stored
/// TODO: this should error if there is a value stored at key, but it isn't an Amount
pub fn amount(key: &str) -> Amount {
    let amount: Option<Amount> = read(key);
    match amount {
        Some(amount) => amount,
        None => Amount::from(0),
    }
}
