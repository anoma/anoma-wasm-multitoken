use anoma_vp_prelude::{
    key::{common, pk_key, SigScheme},
    log_string, read_post, read_pre, storage,
    token::Amount,
    validity_predicate, Address, BTreeSet, BorshDeserialize, BorshSerialize, Signed,
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

fn validate_tx_aux(
    tx_data: Vec<u8>,
    vp_addr: Address,
    keys_changed: BTreeSet<storage::Key>,
    _verifiers: BTreeSet<Address>,
) -> Result<bool> {
    let signed: Signed<multitoken::Op> = signed::extract_signed(&tx_data[..])?;

    match signed.data {
        multitoken::Op::Mint(ref mint) => {
            log("deserialized Mint operation");

            verify_signature_against_pk(&vp_addr, &signed)?;
            log("Verified signature of tx_data against the multitoken's public key");

            let balance_key = mint.balance_key();
            if !only_contains(&keys_changed, &balance_key) {
                log(&format!(
                    "Expected only {} to have changed, but actual keys changed was: {:?}",
                    balance_key, &keys_changed
                ));
                return Ok(false);
            }

            let balance_pre: Option<Amount> = read_pre(&balance_key.to_string());
            let balance_pre = match balance_pre {
                Some(balance) => {
                    log(&format!("pre-existing balance found - {}", balance));
                    balance
                }
                None => {
                    log("no pre-existing balance found");
                    Amount::from(0)
                }
            };

            let balance_post: Option<Amount> = read_post(&balance_key.to_string());
            let balance_post = match balance_post {
                Some(balance) => {
                    log(&format!("new balance found - {}", balance));
                    balance
                }
                None => {
                    log("no new balance found");
                    return Ok(false);
                }
            };

            let mut balance = balance_pre;
            balance.receive(&mint.amount);
            log(&format!("expected new balance - {}", &balance));
            if balance != balance_post {
                log("new balance does not match pre-existing balance with mint applied");
                return Ok(false);
            }
        }
        multitoken::Op::Burn(ref burn) => {
            log("deserialized Burn operation");

            verify_signature_against_pk(&vp_addr, &signed)?;
            log("Verified signature of tx_data against the multitoken's public key");

            let balance_key = burn.balance_key();
            if !only_contains(&keys_changed, &balance_key) {
                log(&format!(
                    "Expected only {} to have changed, but actual keys changed was: {:?}",
                    balance_key, &keys_changed
                ));
                return Ok(false);
            }

            let balance_pre: Option<Amount> = read_pre(&balance_key.to_string());
            let balance_pre = match balance_pre {
                Some(balance) => {
                    log(&format!("pre-existing balance found - {}", balance));
                    balance
                }
                None => {
                    log("no pre-existing balance found");
                    Amount::from(0)
                }
            };

            let balance_post: Option<Amount> = read_post(&balance_key.to_string());
            let balance_post = match balance_post {
                Some(balance) => {
                    log(&format!("new balance found - {}", balance));
                    balance
                }
                None => {
                    log("no new balance found");
                    return Ok(false);
                }
            };

            let mut balance = balance_pre;
            balance.spend(&burn.amount);
            log(&format!("expected new balance - {}", &balance));
            if balance != balance_post {
                log("new balance does not match pre-existing balance with burn applied");
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn only_contains<Orderable: Ord>(set: &BTreeSet<Orderable>, element: &Orderable) -> bool {
    set.contains(element) && set.len() == 1
}

fn verify_signature_against_pk<B: BorshDeserialize + BorshSerialize>(
    addr: &Address,
    signed: &Signed<B>,
) -> Result<()> {
    let pk_storage_key = pk_key(addr);
    let pk: Option<common::PublicKey> = read_pre(&pk_storage_key.to_string());
    let pk = match pk {
        Some(pk) => pk,
        None => return Err(eyre!("couldn't read VP's public key from storage")),
    };
    common::SigScheme::verify_signature(&pk, &signed.data, &signed.sig)
        .wrap_err_with(|| eyre!("verifying signature"))
}
