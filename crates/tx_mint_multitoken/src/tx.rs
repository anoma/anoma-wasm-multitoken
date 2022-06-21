use anoma_tx_prelude::{log_string, read, token::Amount, transaction, write};
use eyre::{eyre, Result};
use shared::{multitoken, signed};

const TX_NAME: &str = "tx_mint_multitoken";

fn log(msg: &str) {
    log_string(format!("[{}] {}", TX_NAME, msg))
}

#[transaction]
fn apply_tx(tx_data: Vec<u8>) {
    if let Err(err) = apply_tx_aux(tx_data) {
        log(&format!("ERROR: {:?}", err));
        panic!("{:?}", err)
    }
}

fn apply_tx_aux(tx_data: Vec<u8>) -> Result<()> {
    log(&format!("called with tx_data - {} bytes", tx_data.len()));

    let op: multitoken::Op = signed::extract_signed(&tx_data[..])?.data;

    let mint = match op {
        multitoken::Op::Mint(mint) => mint,
        _ => return Err(eyre!("expected a mint operation")),
    };

    let balance_key = mint.balance_key().to_string();
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
    balance.receive(&mint.amount);
    write(&balance_key, balance);
    log(&format!("new balance - {}", balance));

    Ok(())
}

#[cfg(test)]
mod tests {

    use anoma::proto::Tx;
    use anoma::types::key::common::SecretKey;
    use anoma_tests::tx::*;
    use anoma_tx_prelude::{address, BorshSerialize, Signed};
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

    #[test]
    fn test_minting_100_red_tokens() {
        tx_host_env::init();
        let multitoken_addr = address::testing::gen_established_address();
        let owner_addr = address::testing::gen_established_address();
        tx_host_env::with(|env| env.spawn_accounts(vec![&multitoken_addr, &owner_addr]));
        const MULTITOKEN_PATH: &str = "multitoken";
        const TOKEN_ID: &str = "blue";
        let amount = Amount::from(100);

        let inner = multitoken::MultitokenAmount {
            multitoken_address: multitoken_addr.encode(),
            multitoken_path: MULTITOKEN_PATH.to_owned(),
            token_id: TOKEN_ID.to_owned(),
            owner_address: owner_addr.encode(),
            amount,
        };
        let mint = multitoken::Op::Mint(inner.clone());
        let inner_sk = random_key();
        let mint = Signed::<multitoken::Op>::new(&inner_sk, mint);

        let data = mint.try_to_vec().unwrap();

        let outer_sk = random_key();
        let tx_data = Tx::new(vec![], Some(data)).sign(&outer_sk).data.unwrap();

        let result = apply_tx_aux(tx_data);

        if let Err(err) = result {
            panic!("apply_tx_aux error: {:?}", err);
        }
        let env = tx_host_env::take();
        assert_eq!(env.all_touched_storage_keys().len(), 1);
        assert!(env
            .all_touched_storage_keys()
            .contains(&inner.balance_key()));
    }
}
