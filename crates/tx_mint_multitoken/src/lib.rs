use anoma_tx_prelude::token::Amount;
use anoma_tx_prelude::Key;
use anoma_tx_prelude::*;
use borsh::de::BorshDeserialize;
use borsh::BorshSerialize;
use eyre::{eyre, Result, WrapErr};
use serde::{Deserialize, Serialize};

const TX_NAME: &str = "tx_mint_multitoken";

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
struct MintMultitoken {
    balance: Key,
    amount: Amount,
}

fn log(msg: &str) {
    log_string(format!("[{}] {}", TX_NAME, msg))
}

#[transaction]
fn apply_tx(tx_data: Vec<u8>) {
    if let Err(err) = apply_tx_aux(tx_data) {
        log(&format!("ERROR: {:?}", err))
    }
}

fn apply_tx_aux(tx_data: Vec<u8>) -> Result<()> {
    log(&format!("called with tx_data - {} bytes", tx_data.len()));
    let signed = SignedTxData::try_from_slice(&tx_data[..])
        .wrap_err_with(|| "deserializing to SignedTxData")?;
    log("deserialized SignedTxData");

    let data = match signed.data {
        Some(data) => data,
        None => return Err(eyre!("no data provided")),
    };
    log(&format!("got data - {} bytes", data.len()));

    let mint_multitoken = MintMultitoken::try_from_slice(&data[..])
        .wrap_err_with(|| "deserializing to MintMultitoken")?;
    log("deserialized MintMultitoken");
    let amount: Option<Amount> = read(mint_multitoken.balance.to_string());
    match amount {
        Some(amount) => {
            log(&format!(
                "storage key already has a value present- {}",
                amount
            ));
        }
        None => {
            log("no existing value found at storage key");
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use anoma::proto::Tx;
    use anoma::types::key::common::SecretKey;
    use anoma_tests::tx::*;
    use rand::prelude::ThreadRng;

    use super::*;

    fn random_key() -> SecretKey {
        let mut rng: ThreadRng = rand::thread_rng();
        let sk: SecretKey = {
            use anoma::types::key::{ed25519, SecretKey, SigScheme};
            ed25519::SigScheme::generate(&mut rng).try_to_sk().unwrap()
        };
        sk
    }

    #[test]
    fn test_deserializing_tx_data() {
        tx_host_env::init();

        let tx_data = vec![];
        assert!(apply_tx_aux(tx_data).is_err());

        let unsigned_data = b"some data".try_to_vec().unwrap();
        let sk = random_key();
        let tx_data = Tx::new(vec![], Some(unsigned_data)).sign(&sk).data.unwrap();
        assert!(apply_tx_aux(tx_data).is_ok());
    }
}
