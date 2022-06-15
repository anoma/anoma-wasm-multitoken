use std::str::FromStr;

use anoma_tx_prelude::Key;

pub const BALANCE_KEY_SEGMENT: &str = "balance";

pub fn balance(
    multitoken_addr: &str,
    multitoken_key_segment: &str,
    token_id: &str,
    owner_addr: &str,
) -> Key {
    Key::from_str(multitoken_addr)
        .unwrap()
        .push(&multitoken_key_segment.to_owned())
        .unwrap()
        .push(&token_id.to_owned())
        .unwrap()
        .push(&BALANCE_KEY_SEGMENT.to_owned())
        .unwrap()
        .push(&owner_addr.to_owned())
        .unwrap()
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
                balance(MULTITOKEN_ADDRESS, "ERC20", ERC20_ADDRESS, OWNER_ADDRESS).to_string()
            )
    }
}
