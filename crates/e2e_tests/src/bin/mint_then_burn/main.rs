use test_runner::chain;
use test_runner::client;
use test_runner::env;
use test_runner::wallet;

mod test;

const TENDERMINT_RPC_ENV_VAR: &str = "ANOMA_LEDGER_ADDRESS";

fn main() {
    chain::join();

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
    let tx_burn_multitoken_path = format!(
        "{}/wasm/tx_burn_multitoken.wasm",
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
        &tx_burn_multitoken_path,
    ) {
        Ok(passed) => {
            if passed {
                std::process::exit(0);
            } else {
                std::process::exit(2);
            }
        }
        Err(err) => {
            eprintln!("Error while running test: {err:?}");
            std::process::exit(1)
        }
    };
}
