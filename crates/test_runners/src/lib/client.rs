use anoma::types::storage::Key;
use borsh::BorshDeserialize;
use eyre::{Context, Result};

use crate::exec::{execute, execute_or_die};
use std::process::Command;

pub struct Client {
    ledger_address: String,
}

impl Client {
    pub fn new(ledger_address: &str) -> Client {
        Client {
            ledger_address: ledger_address.to_owned(),
        }
    }

    pub fn init_account(&self, source: &str, alias: &str, code_path: Option<&str>) {
        let mut cmd = Command::new("anomac");
        let mut args = vec![
            "init-account",
            "--ledger-address",
            &self.ledger_address,
            "--source",
            source,
            "--public-key",
            source,
            "--alias",
            alias,
        ];
        if code_path.is_some() {
            args.append(&mut vec!["--code-path", code_path.unwrap()]);
        };
        let cmd = cmd.args(args);
        execute_or_die(cmd);
    }

    pub fn get_xan_from_faucet(&self, target: &str) {
        let mut cmd = Command::new("anomac");
        let cmd = cmd.args([
            "transfer",
            "--ledger-address",
            &self.ledger_address,
            "--token",
            "XAN",
            "--amount",
            "1000",
            "--source",
            "faucet",
            "--target",
            target,
            "--signer",
            target,
        ]);
        execute_or_die(cmd);
    }

    pub fn tx(&self, code_path: &str, data_path: &str, signer: &str) {
        let mut cmd = Command::new("anomac");
        let cmd = cmd.args([
            "tx",
            "--ledger-address",
            &self.ledger_address,
            "--code-path",
            code_path,
            "--data-path",
            data_path,
            "--signer",
            signer,
        ]);
        execute_or_die(cmd);
    }

    pub fn query_bytes<T: BorshDeserialize>(&self, storage_key: &Key) -> Result<T> {
        let mut cmd = Command::new("anomac");
        let cmd = cmd.env("ANOMA_LOG", "none").args([
            "query-bytes",
            "--ledger-address",
            &self.ledger_address,
            "--storage-key",
            &storage_key.to_string(),
        ]);
        let output = execute(cmd)?;
        let stdout = output.stdout;

        let stdout_str = String::from_utf8(stdout)?;
        let stdout_trimmed = stdout_str.trim().to_owned();
        let borsh_serialized = hex::decode(stdout_trimmed)?;
        T::try_from_slice(&borsh_serialized).wrap_err("couldn't parse stored value".to_string())
    }
}

/// NB: requires ANOMA_NETWORK_CONFIGS_SERVER in env
pub fn join_network(chain_id: &str) -> Result<std::process::Output, std::io::Error> {
    let mut cmd = Command::new("anomac");
    let cmd = cmd.args(["utils", "join-network", "--chain-id", chain_id]);
    execute(cmd)
}

pub fn fetch_wasms_or_die(chain_id: &str) {
    let mut cmd = Command::new("anomac");
    let cmd = cmd.args(["utils", "fetch-wasms", "--chain-id", chain_id]);
    execute_or_die(cmd);
}
