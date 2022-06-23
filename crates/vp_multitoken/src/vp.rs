use anoma_vp_prelude::{
    key::{common, pk_key, SigScheme},
    log_string, read_bytes_pre, storage, validity_predicate, Address, BTreeSet, BorshDeserialize,
    BorshSerialize, Signed,
};
use eyre::{eyre, Context, Result};
use shared::{multitoken, signed};

const VP_NAME: &str = "vp_multitoken";

fn log(msg: &str) {
    log_string(format!("[{}] {}", VP_NAME, msg))
}

#[validity_predicate]
fn validate_tx(
    tx_data: Vec<u8>,
    vp_addr: Address,
    keys_changed: BTreeSet<storage::Key>,
    verifiers: BTreeSet<Address>,
) -> bool {
    log(&format!(
        "validate_tx called with addr: {}, keys_changed: {:#?}, tx_data: \
         {} bytes, verifiers: {:?}",
        vp_addr,
        keys_changed,
        tx_data.len(),
        verifiers
    ));
    match validate_tx_aux(tx_data, vp_addr, keys_changed, verifiers) {
        Ok(result) => result,
        Err(err) => {
            log(&format!("ERROR: {:?}", err));
            panic!("{:?}", err);
        }
    }
}

/// Should return an error iff we were unable to validate a transaction due to something unexpected
fn validate_tx_aux(
    tx_data: Vec<u8>,
    vp_addr: Address,
    keys_changed: BTreeSet<storage::Key>,
    _verifiers: BTreeSet<Address>,
) -> Result<bool> {
    // TODO: could this sometimes be an actual error with the ledger, rather than just tx_data being invalid
    // so we treat it as an error rather than a reject
    let signed: Signed<multitoken::Op> = signed::extract_signed(&tx_data[..])?;

    match signed.data {
        multitoken::Op::Mint(ref mint) => {
            log("deserialized Mint operation");

            if !verify_signature_against_pk(&vp_addr, &signed)? {
                log("Signature did not verify against the multitoken's public key");
                return Ok(false);
            };
            log("Verified signature of tx_data against the multitoken's public key");

            let balance_key = mint.balance_key();
            let supply_key = mint.supply_key();

            let mut expected_keys_changed = BTreeSet::<storage::Key>::new();
            expected_keys_changed.insert(balance_key.clone());
            expected_keys_changed.insert(supply_key.clone());
            if !keys_changed.eq(&expected_keys_changed) {
                log(&format!(
                    "Expected only {:?} to have changed, but actual keys changed was: {:?}",
                    &expected_keys_changed, &keys_changed
                ));
                return Ok(false);
            }

            let (balance_pre, balance_post) = crate::read::amount(&balance_key.to_string())?;
            log(&format!("pre-existing balance - {}", balance_pre));
            log(&format!("new balance - {}", balance_post));

            let mut balance_calculated = balance_pre;
            balance_calculated.receive(&mint.amount);
            log(&format!("expected new balance - {}", &balance_calculated));
            if balance_calculated != balance_post {
                log("new balance does not match pre-existing balance with mint applied");
                return Ok(false);
            }

            let (supply_pre, supply_post) = crate::read::amount(&supply_key.to_string())?;
            log(&format!("pre-existing supply - {}", supply_pre));
            log(&format!("new supply - {}", supply_post));

            let mut supply_calculated = supply_pre;
            supply_calculated.receive(&mint.amount);
            log(&format!("expected new supply - {}", &supply_calculated));
            if supply_calculated != supply_post {
                log("new supply does not match pre-existing supply with mint applied");
                return Ok(false);
            }
            Ok(true)
        }
        multitoken::Op::Burn(ref burn) => {
            log("deserialized Burn operation");

            if !verify_signature_against_pk(&vp_addr, &signed)? {
                log("Signature did not verify against the multitoken's public key");
                return Ok(false);
            };
            log("Verified signature of tx_data against the multitoken's public key");

            let balance_key = burn.balance_key();
            let supply_key = burn.supply_key();

            let mut expected_keys_changed = BTreeSet::<storage::Key>::new();
            expected_keys_changed.insert(balance_key.clone());
            expected_keys_changed.insert(supply_key.clone());
            if !keys_changed.eq(&expected_keys_changed) {
                log(&format!(
                    "Expected only {:?} to have changed, but actual keys changed was: {:?}",
                    &expected_keys_changed, &keys_changed
                ));
                return Ok(false);
            }

            let (balance_pre, balance_post) = crate::read::amount(&balance_key.to_string())?;
            log(&format!("pre-existing balance - {}", balance_pre));
            log(&format!("new balance - {}", balance_post));

            let mut balance_calculated = balance_pre;
            balance_calculated.spend(&burn.amount);
            log(&format!("expected new balance - {}", &balance_calculated));
            if balance_calculated != balance_post {
                log("new balance does not match pre-existing balance with burn applied");
                return Ok(false);
            }

            let (supply_pre, supply_post) = crate::read::amount(&supply_key.to_string())?;
            log(&format!("pre-existing supply - {}", supply_pre));
            log(&format!("new supply - {}", supply_post));

            let mut supply_calculated = supply_pre;
            supply_calculated.spend(&burn.amount);
            log(&format!("expected new supply - {}", &supply_calculated));
            if supply_calculated != supply_post {
                log("new supply does not match pre-existing supply with burn applied");
                return Ok(false);
            }
            Ok(true)
        }
    }
}

// TODO: right now we can't easily differentiate between a signature not verifying and an error
// so this only returns Ok(true) or Err(_)
fn verify_signature_against_pk<B: BorshDeserialize + BorshSerialize>(
    addr: &Address,
    signed: &Signed<B>,
) -> Result<bool> {
    let pk_storage_key = pk_key(addr);
    let pk = match read_bytes_pre(&pk_storage_key.to_string()) {
        Some(bytes) => bytes,
        None => return Err(eyre!("{} had no associated public key", VP_NAME)),
    };
    let pk = common::PublicKey::try_from_slice(&pk[..])?;
    match common::SigScheme::verify_signature(&pk, &signed.data, &signed.sig)
        .wrap_err_with(|| eyre!("verifying signature"))
    {
        Ok(()) => Ok(true),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use anoma_tests::{
        tx::{tx_host_env, TestTxEnv},
        vp::vp_host_env,
    };
    use anoma_vp_prelude::{
        address, key::RefTo, storage, token::Amount, Address, BTreeSet, BorshSerialize, Signed,
    };
    use shared::multitoken;

    use crate::vp::validate_tx;

    use anoma::{proto::Tx, types::key::common::SecretKey};
    use rand::prelude::ThreadRng;

    fn random_key() -> SecretKey {
        let mut rng: ThreadRng = rand::thread_rng();
        let sk: SecretKey = {
            use anoma::types::key::{ed25519, SecretKey, SigScheme};
            ed25519::SigScheme::generate(&mut rng).try_to_sk().unwrap()
        };
        sk
    }

    #[test]
    fn test_mint_disallowed_key_changed() {
        let mut tx_env = TestTxEnv::default();

        let vp_owner = address::testing::established_address_1();
        let user = address::testing::established_address_2();
        let token = address::xan();
        // allowance must be enough to cover the gas costs of any txs made in this test
        let allowance = Amount::from(10_000_000);

        tx_env.spawn_accounts([&vp_owner, &user, &token]);
        tx_env.credit_tokens(&user, &token, allowance);
        let privileged_sk = random_key();
        tx_env.write_public_key(&vp_owner, &privileged_sk.ref_to());

        let mint = multitoken::MultitokenAmount {
            multitoken_address: vp_owner.encode(),
            multitoken_path: "multitoken".to_owned(),
            token_id: "red".to_owned(),
            owner_address: user.encode(),
            amount: Amount::from(50_000_000),
        };
        vp_host_env::init_from_tx(vp_owner.clone(), tx_env, |_| {
            tx_host_env::write(mint.balance_key().to_string(), Amount::from(50_000_000));
            tx_host_env::write(mint.supply_key().to_string(), Amount::from(50_000_000));
            let other = storage::Key::from_str(&format!("#{}", vp_owner.encode()))
                .unwrap()
                .push(&"some arbitary key segment".to_string())
                .unwrap();
            tx_host_env::write(other.to_string(), "some arbitrary value");
        });

        let vp_env = vp_host_env::take();

        let mint = multitoken::Op::Mint(mint);
        let mint = Signed::<multitoken::Op>::new(&privileged_sk, mint);
        let mint = mint.try_to_vec().unwrap();
        let tx = Tx::new(vec![], Some(mint)).sign(&random_key());

        let keys_changed: BTreeSet<storage::Key> = vp_env.all_touched_storage_keys();
        let verifiers: BTreeSet<Address> = BTreeSet::default();
        vp_host_env::set(vp_env);
        assert!(!validate_tx(
            tx.data.unwrap(),
            vp_owner,
            keys_changed,
            verifiers
        ));
    }

    #[test]
    fn test_mint() {
        let mut tx_env = TestTxEnv::default();

        let vp_owner = address::testing::established_address_1();
        let user = address::testing::established_address_2();
        let token = address::xan();
        // allowance must be enough to cover the gas costs of any txs made in this test
        let allowance = Amount::from(10_000_000);

        tx_env.spawn_accounts([&vp_owner, &user, &token]);
        tx_env.credit_tokens(&user, &token, allowance);
        let privileged_sk = random_key();
        tx_env.write_public_key(&vp_owner, &privileged_sk.ref_to());

        let mint = multitoken::MultitokenAmount {
            multitoken_address: vp_owner.encode(),
            multitoken_path: "multitoken".to_owned(),
            token_id: "red".to_owned(),
            owner_address: user.encode(),
            amount: Amount::from(50_000_000),
        };
        vp_host_env::init_from_tx(vp_owner.clone(), tx_env, |_| {
            tx_host_env::write(mint.balance_key().to_string(), Amount::from(50_000_000));
            tx_host_env::write(mint.supply_key().to_string(), Amount::from(50_000_000));
        });

        let vp_env = vp_host_env::take();

        let mint = multitoken::Op::Mint(mint);
        let mint = Signed::<multitoken::Op>::new(&privileged_sk, mint);
        let mint = mint.try_to_vec().unwrap();
        let tx = Tx::new(vec![], Some(mint)).sign(&random_key());

        let keys_changed: BTreeSet<storage::Key> = vp_env.all_touched_storage_keys();
        let verifiers: BTreeSet<Address> = BTreeSet::default();
        vp_host_env::set(vp_env);
        assert!(validate_tx(
            tx.data.unwrap(),
            vp_owner,
            keys_changed,
            verifiers
        ));
    }
}
