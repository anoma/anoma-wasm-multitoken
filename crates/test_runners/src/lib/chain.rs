use std::path::Path;

use crate::{client, wallet};
use retry::{delay::Fixed, retry};

const CHAIN_ID_ENV_VAR: &str = "ANOMA_CHAIN_ID";

pub fn join() {
    let start_delay = std::time::Duration::new(10, 0);

    let chain_id = std::env::var(CHAIN_ID_ENV_VAR)
        .or_else(|_| std::fs::read_to_string("chain-id"))
        .unwrap();
    let chain_id = chain_id.trim();
    println!("Chain ID - {}", chain_id);
    std::thread::sleep(start_delay);
    ensure_joined_or_exit(chain_id);

    // NB: this step is required temporarily until <https://github.com/anoma/namada/issues/98> is resolved
    {
        use fs_extra::dir;

        let mut options = dir::CopyOptions::new(); //Initialize default values for CopyOptions
        options.skip_exist = true;

        let from = Path::new("wasm");
        let to = format!(".anoma/{}", &chain_id);
        let to = Path::new(&to);
        println!("Copying {:?} to {:?}", from, to);
        dir::move_dir(from, to, &options).unwrap();
    }
}

pub fn join_or_exit(chain_id: &str) {
    retry(
        Fixed::from_millis(1000).take(10),
        || match client::join_network(chain_id) {
            Ok(_) => Ok("joined chain"),
            Err(err) => Err(format!("couldn't join chain: {:?}", err)),
        },
    )
    .unwrap();
}

pub fn ensure_joined_or_exit(chain_id: &str) {
    let chain_dir = format!(".anoma/{}", chain_id);
    let chain_dir = Path::new(&chain_dir);
    if chain_dir.exists() {
        println!(
            "Chain dir {} already exists, not attempting to join chain",
            chain_dir.to_string_lossy()
        )
    } else {
        println!(
            "Chain dir {} doesn't exist, will join chain",
            chain_dir.to_string_lossy()
        );
        join_or_exit(chain_id);
    }
}

/// Sets up any accounts needed for the test
pub fn provision_chain(
    client: &client::Client,
    vp_multitoken_path: &str,
    vp_implicit_alias: &str,
    vp_alias: &str,
    owner_implicit_alias: &str,
    owner_alias: &str,
) {
    wallet::gen_address_or_die(vp_implicit_alias);
    client.init_account(vp_implicit_alias, vp_alias, Some(vp_multitoken_path));

    // TODO: setting up this "owner" account should only be done if necessary for the test
    wallet::gen_address_or_die(owner_implicit_alias);
    client.init_account(owner_implicit_alias, owner_alias, None);
    // owner may need to make transactions - get some XAN so that we can pay gas
    client.get_xan_from_faucet(owner_alias);
}
