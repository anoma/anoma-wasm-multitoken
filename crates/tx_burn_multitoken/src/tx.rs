use anoma_tx_prelude::{log_string, read, token::Amount, transaction, write};
use eyre::{eyre, Result};
use shared::{multitoken, signed};

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

    match op {
        multitoken::Op::Burn(burn) => {
            let balance_key = burn.balance_key().to_string();
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
            balance.spend(&burn.amount);
            write(&balance_key, balance);
            log(&format!("new balance - {}", balance));
        }
        _ => return Err(eyre!("expected a burn operation")),
    }

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
