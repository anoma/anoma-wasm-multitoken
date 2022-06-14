use anoma::types::key::common;
use anoma::types::key::ed25519;
use anoma::types::key::SecretKey;
use anoma::types::key::SigScheme;

pub fn random_key() -> common::SecretKey {
    let mut rng = rand::thread_rng();
    ed25519::SigScheme::generate(&mut rng).try_to_sk().unwrap()
}
