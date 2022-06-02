use anoma_tx_prelude::*;
use eyre::Result;

#[transaction]
fn apply_tx(tx_data: Vec<u8>) {
    apply_tx_aux(tx_data).unwrap();
}

fn apply_tx_aux(tx_data: Vec<u8>) -> Result<()> {
    log_string(format!("apply_tx called with data: {:#?}", tx_data));
    Ok(())
}

#[cfg(test)]
mod tests {
    use anoma_tests::tx::*;

    use super::*;

    /// An example test, checking that this transaction performs no storage
    /// modifications.
    #[test]
    fn test_no_op_transaction() {
        // The environment must be initialized first
        tx_host_env::init();

        let tx_data = vec![];
        apply_tx(tx_data);

        let env = tx_host_env::take();
        assert!(env.all_touched_storage_keys().is_empty());
    }
}
