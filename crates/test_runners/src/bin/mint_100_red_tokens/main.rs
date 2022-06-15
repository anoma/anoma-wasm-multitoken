use std::process::Command;

fn main() {
    Command::new("anomac")
        .args(["--ledger-address", "$ANOMA_LEDGER_ADDRESS", "epoch"])
        .output()
        .expect("failed to execute process");
}
