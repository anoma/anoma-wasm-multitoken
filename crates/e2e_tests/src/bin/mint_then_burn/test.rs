use eyre::Result;
use namada::{proto::Signed, types::token::Amount};
use shared::keys::MULTITOKEN_PATH;
use shared::{keys, multitoken};
use test_runner::{client, file, wallet};

const TOKEN_ID: &str = "red";

pub(crate) fn run(
    client: &client::Client,
    vp_implicit_alias: &str,
    vp_alias: &str,
    owner_alias: &str,
    tx_mint_multitoken_path: &str,
    tx_burn_multitoken_path: &str,
) -> Result<bool> {
    let multitoken_address = wallet::find_address(vp_alias)?;
    let owner_address = wallet::find_address(owner_alias)?;
    let privileged_sk = wallet::read_secret_key(vp_implicit_alias)?;

    let mint = multitoken::MultitokenAmount {
        multitoken_address: wallet::find_address(vp_alias)?,
        multitoken_path: MULTITOKEN_PATH.to_owned(),
        token_id: TOKEN_ID.to_owned(),
        owner_address: wallet::find_address(owner_alias)?,
        amount: Amount::from(50_000_000),
    };
    let mint = multitoken::Op::Mint(mint);
    let mint = Signed::<multitoken::Op>::new(&privileged_sk, mint);
    let mint_file = file::write_temporary(mint)?;

    let burn = multitoken::MultitokenAmount {
        multitoken_address: wallet::find_address(vp_alias)?,
        multitoken_path: MULTITOKEN_PATH.to_owned(),
        token_id: TOKEN_ID.to_owned(),
        owner_address: wallet::find_address(owner_alias)?,
        amount: Amount::from(10_000_000),
    };
    let burn = multitoken::Op::Burn(burn);
    let burn = Signed::<multitoken::Op>::new(&privileged_sk, burn);
    let burn_file = file::write_temporary(burn)?;

    let balance_key = keys::balance(
        &multitoken_address,
        MULTITOKEN_PATH,
        TOKEN_ID,
        &owner_address,
    );
    let supply_key = keys::supply(&multitoken_address, MULTITOKEN_PATH, TOKEN_ID);

    client.tx(
        tx_mint_multitoken_path,
        owner_alias,
        Some(mint_file.path().to_string_lossy().to_string().as_str()),
    );
    let expected = Amount::from(50_000_000);

    let balance: Amount = client.query_bytes(&balance_key)?;
    if balance != expected {
        eprintln!("balance: got {}, wanted {}", balance, expected);
        return Ok(false);
    }
    let supply: Amount = client.query_bytes(&supply_key)?;
    if supply != expected {
        eprintln!("supply: got {}, wanted {}", supply, expected);
        return Ok(false);
    }

    client.tx(
        tx_burn_multitoken_path,
        owner_alias,
        Some(burn_file.path().to_string_lossy().to_string().as_str()),
    );
    let expected = Amount::from(40_000_000);

    let balance: Amount = client.query_bytes(&balance_key)?;
    if balance != expected {
        eprintln!("balance: got {}, wanted {}", balance, expected);
        return Ok(false);
    }
    let supply: Amount = client.query_bytes(&supply_key)?;
    if supply != expected {
        eprintln!("supply: got {}, wanted {}", supply, expected);
        return Ok(false);
    }

    Ok(true)
}
