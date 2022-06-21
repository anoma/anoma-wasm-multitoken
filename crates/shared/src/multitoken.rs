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

    pub fn supply_key(&self) -> Key {
        keys::supply(
            self.multitoken_address.as_str(),
            self.multitoken_path.as_str(),
            &self.token_id,
        )
    }
}

/// Represents possible transactions that may be made against vp_multitoken
#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum Op {
    Mint(MultitokenAmount),
    Burn(MultitokenAmount),
}

#[cfg(test)]
mod tests {
    use anoma::types::storage::DbKeySeg;
    use anoma_tx_prelude::{token::Amount, Address};

    use crate::keys::SUPPLY_KEY_SEGMENT;

    use super::MultitokenAmount;

    const MULTITOKEN_ADDRESS: &str =
        "atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn";
    const MULTITOKEN_PATH: &str = "ERC20";
    const DAI_ERC20_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
    const OWNER_ADDRESS: &str =
        "atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4";

    #[test]
    pub fn test_balance_key() {
        let m = MultitokenAmount {
            multitoken_address: MULTITOKEN_ADDRESS.to_owned(),
            multitoken_path: MULTITOKEN_PATH.to_owned(),
            token_id: DAI_ERC20_ADDRESS.to_owned(),
            owner_address: OWNER_ADDRESS.to_owned(),
            amount: Amount::from(100),
        };
        assert!(matches!(
            &m.balance_key().segments[..],
            [
                DbKeySeg::AddressSeg(multitoken_addr),
                DbKeySeg::StringSeg(multitoken_path),
                DbKeySeg::StringSeg(token_id),
                DbKeySeg::StringSeg(balance_key_seg),
                DbKeySeg::AddressSeg(owner_addr),
            ] if multitoken_addr == &Address::decode(MULTITOKEN_ADDRESS).unwrap() &&
            multitoken_path == MULTITOKEN_PATH &&
            token_id == DAI_ERC20_ADDRESS &&
            balance_key_seg == crate::keys::BALANCE_KEY_SEGMENT &&
            owner_addr == &Address::decode(OWNER_ADDRESS).unwrap()
        ))
    }

    #[test]
    pub fn test_supply_key() {
        let m = MultitokenAmount {
            multitoken_address: MULTITOKEN_ADDRESS.to_owned(),
            multitoken_path: MULTITOKEN_PATH.to_owned(),
            token_id: DAI_ERC20_ADDRESS.to_owned(),
            owner_address: OWNER_ADDRESS.to_owned(),
            amount: Amount::from(100),
        };
        assert!(matches!(
            &m.supply_key().segments[..],
            [
                DbKeySeg::AddressSeg(multitoken_addr),
                DbKeySeg::StringSeg(multitoken_path),
                DbKeySeg::StringSeg(token_id),
                DbKeySeg::StringSeg(supply_key_seg),
            ] if multitoken_addr == &Address::decode(MULTITOKEN_ADDRESS).unwrap() &&
            multitoken_path == MULTITOKEN_PATH &&
            token_id == DAI_ERC20_ADDRESS &&
            supply_key_seg == SUPPLY_KEY_SEGMENT
        ))
    }
}
