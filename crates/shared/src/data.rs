use crate::keys;
use anoma_tx_prelude::token::Amount;
use anoma_tx_prelude::BorshDeserialize;
use anoma_tx_prelude::Key;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct MintMultitoken {
    pub multitoken_address: String,
    pub multitoken_key: String,
    pub token_id: String,
    pub owner_address: String,
    pub amount: Amount,
}

impl MintMultitoken {
    pub fn balance_key(&self) -> Key {
        keys::balance(
            self.multitoken_address.as_str(),
            self.multitoken_key.as_str(),
            &self.token_id,
            &self.owner_address,
        )
    }
}
