use std::path::Path;

use crate::{chain, client, wallet};
use retry::{delay::Fixed, retry};

pub fn join_or_exit(chain_id: &str) {
    retry(
        Fixed::from_millis(1000).take(10),
        || match client::join_network(chain_id) {
            Ok(_) => Ok("joined chain"),
            Err(err) => Err(format!("couldn't join chain: {:?}", err)),
        },
    )
    .unwrap();
    client::fetch_wasms_or_die(chain_id);
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
        chain::join_or_exit(chain_id);
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
