use namada::types::key::common;
use namada::types::key::ed25519;
use namada::types::key::SecretKey;
use namada::types::key::SigScheme;

pub fn random_key() -> common::SecretKey {
    let mut rng = rand::thread_rng();
    ed25519::SigScheme::generate(&mut rng).try_to_sk().unwrap()
}
