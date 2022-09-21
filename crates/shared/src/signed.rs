//! Utilities to help with unwrapping and checking signed data

use eyre::{eyre, Context, Result};
use namada_tx_prelude::{BorshDeserialize, BorshSerialize, Signed, SignedTxData};

/// Extracts and serializes the data field from a Borsh-serialized SignedTxData
/// We cannot check the outer SignedTxData signature field in our wasm code as that is calculated over
/// more than just tx_data
pub fn extract_signed<T: BorshSerialize + BorshDeserialize>(tx_data: &[u8]) -> Result<Signed<T>> {
    let signed =
        SignedTxData::try_from_slice(tx_data).wrap_err_with(|| "deserializing to SignedTxData")?;

    let data = match signed.data {
        Some(data) => data,
        None => return Err(eyre!("no data provided")),
    };
    Signed::<T>::try_from_slice(&data[..]).wrap_err_with(|| "deserializing to Signed")
}
