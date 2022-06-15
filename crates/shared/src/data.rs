use std::str::FromStr;

use crate::BALANCE_KEY_SEGMENT;
use anoma_tx_prelude::token::Amount;
use anoma_tx_prelude::BorshDeserialize;
use anoma_tx_prelude::Key;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct MintMultitoken {
    pub multitoken_address: String,
    pub multitoken_key: Key,
    pub token_id: String,
    pub owner_address: String,
    pub amount: Amount,
}

impl MintMultitoken {
    pub fn balance_key(&self) -> Key {
        Key::from_str(self.multitoken_address.as_str())
            .unwrap()
            .join(&self.multitoken_key)
            .push(&self.token_id)
            .unwrap()
            .push(&BALANCE_KEY_SEGMENT.to_owned())
            .unwrap()
            .push(&self.owner_address)
            .unwrap()
    }
}
