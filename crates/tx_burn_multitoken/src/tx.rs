use eyre::{eyre, Result};
use namada_tx_prelude::{log_string, transaction};
use shared::{multitoken, signed, update};

const TX_NAME: &str = "tx_burn_multitoken";

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

    let burn = match op {
        multitoken::Op::Burn(burn) => burn,
        _ => return Err(eyre!("expected a burn operation")),
    };

    let balance_key = burn.balance_key();
    update::amount(&balance_key, |amount| {
        log(&format!("existing value for {} is {}", balance_key, amount));
        amount.spend(&burn.amount);
        log(&format!("new value for {} will be {}", balance_key, amount));
    })?;

    let supply_key = burn.supply_key();
    update::amount(&supply_key, |amount| {
        log(&format!("existing value for {} is {}", supply_key, amount));
        amount.spend(&burn.amount);
        log(&format!("new value for {} will be {}", supply_key, amount));
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use namada_tests::tx::*;
    use namada_tx_prelude::{address, token::Amount, BorshSerialize, Signed};

    use namada::proto::Tx;
    use namada::types::key::common::SecretKey;

    use rand::prelude::ThreadRng;

    use super::*;

    fn random_key() -> SecretKey {
        let mut rng: ThreadRng = rand::thread_rng();
        let sk: SecretKey = {
            use namada::types::key::{ed25519, SecretKey, SigScheme};
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
    fn test_burn_40_of_50_tokens() {
        let multitoken = address::testing::gen_established_address();
        let user = address::testing::gen_established_address();
        let token = address::xan();
        const MULTITOKEN_PATH: &str = "multitoken";
        const TOKEN_ID: &str = "blue";
        let initial = Amount::from(50);
        let to_burn = Amount::from(40);
        let inner = multitoken::MultitokenAmount {
            multitoken_address: multitoken.encode(),
            multitoken_path: MULTITOKEN_PATH.to_owned(),
            token_id: TOKEN_ID.to_owned(),
            owner_address: user.encode(),
            amount: to_burn,
        };
        tx_host_env::init();
        tx_host_env::with(|tx_env| {
            tx_env.spawn_accounts(vec![&multitoken, &user, &token]);
            tx_env
                .storage
                .write(&inner.balance_key(), initial.try_to_vec().unwrap())
                .unwrap();
            tx_env
                .storage
                .write(&inner.supply_key(), initial.try_to_vec().unwrap())
                .unwrap();
        });
        let burn = multitoken::Op::Burn(inner.clone());
        let inner_sk = random_key();
        let burn = Signed::<multitoken::Op>::new(&inner_sk, burn);
        let data = burn.try_to_vec().unwrap();
        let outer_sk = random_key();
        let tx_data = Tx::new(vec![], Some(data)).sign(&outer_sk).data.unwrap();

        let result = apply_tx_aux(tx_data);

        if let Err(err) = result {
            panic!("apply_tx_aux error: {:?}", err);
        }
        let env = tx_host_env::take();
        assert_eq!(env.all_touched_storage_keys().len(), 2);
        assert!(env
            .all_touched_storage_keys()
            .contains(&inner.balance_key()));
        assert!(env.all_touched_storage_keys().contains(&inner.supply_key()));
    }
}
