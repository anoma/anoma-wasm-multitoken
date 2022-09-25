use std::str::FromStr;

use namada_tx_prelude::Key;

pub const BALANCE_KEY_SEGMENT: &str = "balance";
pub const SUPPLY_KEY_SEGMENT: &str = "supply";

/// This is the path from the top-level address to the multitoken hierarchy
/// It could differ between multitokens e.g. in the Ethereum bridge, it would be "ERC20"
pub const MULTITOKEN_PATH: &str = "multitoken";

pub fn balance(
    multitoken_addr: &str,
    multitoken_path: &str,
    token_id: &str,
    owner_addr: &str,
) -> Key {
    Key::from_str(&format!("#{}", multitoken_addr))
        .unwrap()
        .push(&multitoken_path.to_owned())
        .unwrap()
        .push(&token_id.to_owned())
        .unwrap()
        .push(&BALANCE_KEY_SEGMENT.to_owned())
        .unwrap()
        .push(&format!("#{}", owner_addr))
        .unwrap()
}

pub fn supply(multitoken_addr: &str, multitoken_path: &str, token_id: &str) -> Key {
    Key::from_str(&format!("#{}", multitoken_addr))
        .unwrap()
        .push(&multitoken_path.to_owned())
        .unwrap()
        .push(&token_id.to_owned())
        .unwrap()
        .push(&SUPPLY_KEY_SEGMENT.to_owned())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use namada::types::storage::DbKeySeg;
    use namada_tx_prelude::Address;

    const MULTITOKEN_ADDRESS: &str =
        "atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn";
    const MULTITOKEN_PATH: &str = "ERC20";
    const DAI_ERC20_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
    const OWNER_ADDRESS: &str =
        "atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4";

    #[test]
    fn test_balance_segment_types() {
        let key = balance(
            MULTITOKEN_ADDRESS,
            MULTITOKEN_PATH,
            DAI_ERC20_ADDRESS,
            OWNER_ADDRESS,
        );
        assert!(matches!(
            &key.segments[..],
            [
                DbKeySeg::AddressSeg(multitoken_addr),
                DbKeySeg::StringSeg(multitoken_path),
                DbKeySeg::StringSeg(token_id),
                DbKeySeg::StringSeg(balance_key_seg),
                DbKeySeg::AddressSeg(owner_addr),
            ] if multitoken_addr == &Address::decode(MULTITOKEN_ADDRESS).unwrap() &&
            multitoken_path == MULTITOKEN_PATH &&
            token_id == DAI_ERC20_ADDRESS &&
            balance_key_seg == BALANCE_KEY_SEGMENT &&
            owner_addr == &Address::decode(OWNER_ADDRESS).unwrap()
        ))
    }

    #[test]
    fn test_balance_to_string() {
        let key = balance(
            MULTITOKEN_ADDRESS,
            MULTITOKEN_PATH,
            DAI_ERC20_ADDRESS,
            OWNER_ADDRESS,
        );
        assert_eq!(
                "#atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn/ERC20/0x6B175474E89094C44Da98b954EedeAC495271d0F/balance/#atest1d9khqw36x9zyxwfhgfpygv2pgc65gse4gy6rjs34gfzr2v69gy6y23zpggurjv2yx5m52sesu6r4y4",
                key.to_string()
            )
    }

    #[test]
    fn test_supply_segment_types() {
        let key = supply(MULTITOKEN_ADDRESS, "ERC20", DAI_ERC20_ADDRESS);
        assert!(matches!(
            &key.segments[..],
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

    #[test]
    fn test_supply_to_string() {
        let key = supply(MULTITOKEN_ADDRESS, "ERC20", DAI_ERC20_ADDRESS);
        assert_eq!(
                "#atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn/ERC20/0x6B175474E89094C44Da98b954EedeAC495271d0F/supply",
                key.to_string(),
            )
    }
}
