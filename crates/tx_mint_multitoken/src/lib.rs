use anoma_tx_prelude::{
    log_string, read, token::Amount, transaction, write, BorshDeserialize, SignedTxData,
};
use eyre::{eyre, Result, WrapErr};
use shared::data;

const TX_NAME: &str = "tx_mint_multitoken";

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

    let mint_multitoken = data::MintMultitoken::try_from_slice(&data[..])
        .wrap_err_with(|| "deserializing to MintMultitoken")?;
    log("deserialized MintMultitoken");

    let balance_key = mint_multitoken.balance_key().to_string();
    let balance: Option<Amount> = read(&balance_key);
    let mut balance = match balance {
        Some(amount) => {
            log(&format!("existing balance found - {}", amount));
            amount
        }
        None => {
            log("no existing balance found");
            Amount::from(0)
        }
    };
    // TODO: this may panic - test what happens when overflow
    balance.receive(&mint_multitoken.amount);
    write(&balance_key, balance);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anoma::proto::Tx;
    use anoma::types::key::common::SecretKey;
    use anoma_tests::tx::*;
    use anoma_tx_prelude::{BorshSerialize, Key};
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
    fn test_cannot_pass_empty_data() {
        tx_host_env::init();

        let tx_data = vec![];
        assert!(apply_tx_aux(tx_data).is_err());

        let env = tx_host_env::take();
        assert!(env.all_touched_storage_keys().is_empty());
    }

    const MULTITOKEN_ADDRESS: &str =
        "atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn";
    const MULTITOKEN_KEY_SEGMENT: &str = "multitoken";
    const TOKEN_KEY_SEGMENT: &str = "red";
    const OWNER_ADDRESS: &str =
        "atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4";

    #[test]
    fn test_minting_100_red_tokens() {
        tx_host_env::init();

        let amount = Amount::from(100);
        let mint = data::MintMultitoken {
            multitoken_address: MULTITOKEN_ADDRESS.to_owned(),
            multitoken_key: Key::from_str(MULTITOKEN_KEY_SEGMENT).unwrap(),
            token_id: TOKEN_KEY_SEGMENT.to_owned(),
            owner_address: OWNER_ADDRESS.to_owned(),
            amount,
        };

        let unsigned_data = mint.try_to_vec().unwrap();

        let sk = random_key();
        let tx_data = Tx::new(vec![], Some(unsigned_data)).sign(&sk).data.unwrap();

        assert!(apply_tx_aux(tx_data).is_ok());

        let env = tx_host_env::take();
        assert_eq!(env.all_touched_storage_keys().len(), 1);
        assert!(env.all_touched_storage_keys().contains(&mint.balance_key()));
    }
}
