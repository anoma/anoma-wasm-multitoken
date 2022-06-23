use anoma_tx_prelude::{log_string, transaction};
use eyre::{eyre, Result};
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

    use anoma_tests::tx::*;

    use super::*;

    #[test]
    fn test_cannot_pass_empty_data() {
        tx_host_env::init();

        let tx_data = vec![];
        assert!(apply_tx_aux(tx_data).is_err());

        let env = tx_host_env::take();
        assert!(env.all_touched_storage_keys().is_empty());
    }
}
