use anyhow::Result;
use assert_cmd::{crate_name, Command};
use predicates::str::contains;

#[test]
pub fn test_cardano_node() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("node");
    cmd.assert().failure().stderr(contains("Manage cardano nodes"));
    cmd.arg("help");
    cmd.assert().success().stdout(contains("Manage cardano nodes"));
    Ok(())
}

#[test]
pub fn test_cardano_node_install() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("node");
    cmd.arg("install");
    cmd.arg("-h");
    cmd.assert()
        .success()
        .stdout(contains("Install the latest cardano-node binary"));
    Ok(())
}
