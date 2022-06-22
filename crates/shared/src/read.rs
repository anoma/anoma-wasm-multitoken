//! Helpers for reading from storage

use anoma_tx_prelude::{read_bytes, token::Amount, BorshDeserialize};
use eyre::{Context, Result};

/// Returns the stored Amount, or 0 if not stored
pub fn amount(key: &str) -> Result<Amount> {
    let bytes = match read_bytes(key) {
        Some(bytes) => bytes,
        None => return Ok(Amount::from(0)),
    };
    Amount::try_from_slice(&bytes[..]).wrap_err("couldn't deserialize to Amount")
}

#[cfg(test)]
mod tests {
    use anoma_tests::tx::*;
    use anoma_tx_prelude::token::Amount;

    use crate::read;

    #[test]
    fn test_amount_returns_zero_for_uninitialized_storage() {
        tx_host_env::init();

        let a = read::amount("some arbitrary key with no stored value").unwrap();
        assert_eq!(a, Amount::from(0));
    }

    #[test]
    fn test_amount_returns_stored_amount() {
        tx_host_env::init();
        let key = "some arbitrary key";
        let amount = Amount::from(1_000_000);
        tx_host_env::write(key, amount);

        let a = read::amount(key).unwrap();
        assert_eq!(a, amount);
    }

    #[test]
    fn test_amount_errors_if_not_amount() {
        tx_host_env::init();
        let key = "some arbitrary key";
        let amount = "not an Amount type";
        tx_host_env::write(key, amount);

        assert!(matches!(read::amount(key), Err(_)))
    }
}
