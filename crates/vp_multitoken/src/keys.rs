//! key helpers
use std::str::FromStr;

use anoma_vp_prelude::storage::{DbKeySeg, Key};

const MULTITOKEN_KEY_SEGMENT: &str = "ERC20";
const BALANCE_KEY_SEGMENT: &str = "balance";

pub fn balance(multitoken_addr: &str, token_id: &str, owner_addr: &str) -> Key {
    Key::from_str(multitoken_addr)
        .unwrap()
        .push(&MULTITOKEN_KEY_SEGMENT.to_owned())
        .unwrap()
        .push(&token_id.to_owned())
        .unwrap()
        .push(&BALANCE_KEY_SEGMENT.to_owned())
        .unwrap()
        .push(&owner_addr.to_owned())
        .unwrap()
}

pub fn is_balance_key(key: &Key) -> bool {
    key.segments.len() == 5
        && key.segments[1] == DbKeySeg::StringSeg(MULTITOKEN_KEY_SEGMENT.to_owned())
        && key.segments[3] == DbKeySeg::StringSeg(BALANCE_KEY_SEGMENT.to_owned())
    // TODO: check whether relevant key segments are syntactically correct bech32m addresses or Ethereum addresses
}

#[cfg(test)]
mod tests {
    use super::*;

    const MULTITOKEN_ADDRESS: &str =
        "atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn";
    const ERC20_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
    const OWNER_ADDRESS: &str =
        "atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4";

    #[test]
    fn test_balance() {
        assert_eq!(
                "atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn/ERC20/0x6B175474E89094C44Da98b954EedeAC495271d0F/balance/atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4",
                balance(MULTITOKEN_ADDRESS, ERC20_ADDRESS, OWNER_ADDRESS).to_string()
            )
    }

    #[test]
    fn test_is_balance_key() {
        let balance = Key::from_str("atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn/ERC20/0x6B175474E89094C44Da98b954EedeAC495271d0F/balance/atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4").unwrap();
        assert!(is_balance_key(&balance))
    }
}
