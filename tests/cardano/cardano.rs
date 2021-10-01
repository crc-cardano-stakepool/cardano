use anyhow::Result;
use assert_cmd::{crate_name, Command};
use predicates::str::contains;

#[test]
pub fn cardano_cli_works() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.assert()
        .failure()
        .stderr(contains("Manage cardano components"));
    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(contains("Manage cardano components"));
    Ok(())
}
