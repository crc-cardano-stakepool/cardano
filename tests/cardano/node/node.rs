use anyhow::Result;
use assert_cmd::{crate_name, Command};
use predicates::str::contains;

#[test]
pub fn cardano_node_works() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("node");
    cmd.assert().failure().stderr(contains("Manage cardano nodes"));
    cmd.arg("help");
    cmd.assert().success().stdout(contains("Manage cardano nodes"));
    Ok(())
}
