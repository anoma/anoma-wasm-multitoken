//! key helpers

use anoma_vp_prelude::storage::Key;
use shared::keys::MULTITOKEN_PATH;

pub fn balance(multitoken_addr: &str, token_id: &str, owner_addr: &str) -> Key {
    shared::keys::balance(multitoken_addr, MULTITOKEN_PATH, token_id, owner_addr)
}
