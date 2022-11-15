//! Helpers for reading from storage

use eyre::{Context, Result};
use namada_tx_prelude::{storage::Key, token::Amount, BorshDeserialize, Ctx, StorageRead};

/// Returns the stored Amount, or 0 if not stored
pub fn amount(ctx: &Ctx, key: &Key) -> Result<Amount> {
    let bytes = match ctx.read_bytes(key).unwrap() {
        // TODO: don't just unwrap
        Some(bytes) => bytes,
        None => return Ok(Amount::from(0)),
    };
    Amount::try_from_slice(&bytes[..]).wrap_err("couldn't deserialize to Amount")
}

#[cfg(test)]
mod tests {
    use namada_tests::tx::*;
    use namada_tx_prelude::{storage::Key, token::Amount, StorageWrite};

    use crate::read;

    #[test]
    fn test_amount_returns_zero_for_uninitialized_storage() {
        tx_host_env::init();

        let key = Key::parse("some arbitrary key with no stored value").unwrap();
        let a = read::amount(tx_host_env::ctx(), &key).unwrap();
        assert_eq!(a, Amount::from(0));
    }

    #[test]
    fn test_amount_returns_stored_amount() {
        tx_host_env::init();
        let key = Key::parse("some arbitrary key").unwrap();
        let amount = Amount::from(1_000_000);
        tx_host_env::ctx().write(&key, amount).unwrap();

        let a = read::amount(tx_host_env::ctx(), &key).unwrap();
        assert_eq!(a, amount);
    }

    #[test]
    fn test_amount_errors_if_not_amount() {
        tx_host_env::init();
        let key = Key::parse("some arbitrary key").unwrap();
        let amount = "not an Amount type";
        tx_host_env::ctx().write(&key, amount).unwrap();

        assert!(matches!(read::amount(tx_host_env::ctx(), &key), Err(_)))
    }
}
