use test_runners::chain;
use test_runners::client;
use test_runners::env;
use test_runners::wallet;

mod test;

const CHAIN_ID_ENV_VAR: &str = "ANOMA_CHAIN_ID";
const TENDERMINT_RPC_ENV_VAR: &str = "ANOMA_LEDGER_ADDRESS";

fn main() {
    let chain_id = env::get_var_or_die(CHAIN_ID_ENV_VAR);
    chain::ensure_joined(&chain_id);

    let vp_implicit_alias = wallet::random_alias("multitoken-implicit");
    let vp_alias = wallet::random_alias("multitoken-established");
    let owner_implicit_alias = wallet::random_alias("owner-implicit");
    let owner_alias = wallet::random_alias("owner-established");

    let ledger_address = env::get_var_or_die(TENDERMINT_RPC_ENV_VAR);
    let current_dir = std::env::current_dir().unwrap();

    let vp_multitoken_path = format!("{}/wasm/vp_multitoken.wasm", current_dir.to_string_lossy());
    let tx_mint_multitoken_path = format!(
        "{}/wasm/tx_mint_multitoken.wasm",
        current_dir.to_string_lossy()
    );

    let client = client::Client::new(&ledger_address);

    chain::provision_chain(
        &client,
        &vp_multitoken_path,
        &vp_implicit_alias,
        &vp_alias,
        &owner_implicit_alias,
        &owner_alias,
    );

    match test::run(
        &client,
        &vp_implicit_alias,
        &vp_alias,
        &owner_alias,
        &tx_mint_multitoken_path,
    ) {
        Ok(passed) => {
            if passed {
                std::process::exit(0);
            } else {
                std::process::exit(2);
            }
        }
        Err(err) => {
            eprintln!("Error while running test: {:?}", err);
            std::process::exit(1)
        }
    };
}
