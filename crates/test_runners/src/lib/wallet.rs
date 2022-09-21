use std::{
    fs::{self, File},
    io::Read,
    process::Command,
};

use borsh::BorshDeserialize;
use eyre::{eyre, Context, Result};
use namada::types::key::common;
use rand::{distributions::Alphanumeric, Rng};

use crate::exec::{execute, execute_or_die};

pub fn read_secret_key(alias: &str) -> Result<common::SecretKey> {
    let mut cmd = Command::new("anomaw");
    let cmd = cmd.args(["key", "export", "--alias", alias]);
    execute(cmd)?;
    let filename = format!("key_{}", alias.to_lowercase());
    let mut f = File::open(&filename).wrap_err_with(|| eyre!("Couldn't open {}", &filename))?;
    let metadata = fs::metadata(&filename)?;
    let mut buf: Vec<u8> = vec![0; metadata.len() as usize];
    f.read_exact(&mut buf)?;
    let deserialized = common::SecretKey::try_from_slice(&buf[..])?;
    Ok(deserialized)
}

pub fn find_address(alias: &str) -> Result<String> {
    let mut cmd = Command::new("anomaw");
    let cmd = cmd.args(["address", "find", "--alias", alias]);
    let output = execute(cmd)?;
    let output = String::from_utf8(output.stdout)?;
    Ok(parse_anomaw_find_address(output))
}

pub fn parse_anomaw_find_address(output: String) -> String {
    // crudely get the bech32m address from stdout
    let components: Vec<_> = output.split(':').collect();
    components[1].trim().to_owned()
}

pub fn gen_address_or_die(alias: &str) {
    let mut cmd = Command::new("anomaw");
    let cmd = cmd.args(["address", "gen", "--unsafe-dont-encrypt", "--alias", alias]);
    execute_or_die(cmd);
}

/// Generates a random alias based on the passed prefix
pub fn random_alias(prefix: &str) -> String {
    let mut alias = prefix.to_string();
    alias.push('-');
    let mut rng = rand::thread_rng();
    let suffix: String = (0..8).map(|_| rng.sample(Alphanumeric) as char).collect();
    alias.push_str(&suffix);
    alias
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_anomaw_find_address() {
        let output = "Found address Established: atest1v4ehgw36g4pyg3j9x3qnjd3cxgmyz3fk8qcrys3hxdp5xwfnx3zyxsj9xgunxsfjg5u5xvzyzrrqtn".to_string();
        parse_anomaw_find_address(output);
    }
}
