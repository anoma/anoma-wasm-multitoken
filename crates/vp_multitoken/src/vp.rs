use anoma_vp_prelude::{
    key::{common, pk_key, SigScheme},
    log_string, read_pre, storage, validity_predicate, Address, BTreeSet, BorshDeserialize,
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

            let mut expected_keys_changed = BTreeSet::<storage::Key>::new();
            expected_keys_changed.insert(balance_key.clone());
            if !keys_changed.eq(&expected_keys_changed) {
                log(&format!(
                    "Expected only {} to have changed, but actual keys changed was: {:?}",
                    &balance_key, &keys_changed
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

            let mut expected_keys_changed = BTreeSet::<storage::Key>::new();
            expected_keys_changed.insert(balance_key.clone());
            if !keys_changed.eq(&expected_keys_changed) {
                log(&format!(
                    "Expected only {} to have changed, but actual keys changed was: {:?}",
                    balance_key, &keys_changed
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
