use anoma_tx_prelude::token::Amount;
use anoma_tx_prelude::BorshDeserialize;
use anoma_tx_prelude::Key;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct MintMultitoken {
    pub balance: Key,
    pub amount: Amount,
}
