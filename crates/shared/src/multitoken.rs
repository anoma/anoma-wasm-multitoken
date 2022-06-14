use crate::keys;
use anoma_tx_prelude::token::Amount;
use anoma_tx_prelude::BorshDeserialize;
use anoma_tx_prelude::Key;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};

/// Represents an amount of a multitoken token for some owner
#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct MultitokenAmount {
    pub multitoken_address: String,
    pub multitoken_path: String,
    pub token_id: String,
    pub owner_address: String,
    pub amount: Amount,
}

impl MultitokenAmount {
    pub fn balance_key(&self) -> Key {
        keys::balance(
            self.multitoken_address.as_str(),
            self.multitoken_path.as_str(),
            &self.token_id,
            &self.owner_address,
        )
    }
}

/// Represents possible transactions that may be made against vp_multitoken
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum Op {
    Mint(MultitokenAmount),
    Burn(MultitokenAmount),
}
