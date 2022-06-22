use anoma_tx_prelude::{log_string, transaction, write};
use eyre::{eyre, Result};
use shared::{multitoken, read, signed};

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
    let balance_key = burn.balance_key().to_string();
    let mut balance = read::amount(&balance_key);
    log(&format!("existing balance is {}", balance));
    balance.spend(&burn.amount);
    write(&balance_key, balance);
    log(&format!("new balance - {}", balance));

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
