use anyhow::Result;
use assert_cmd::{crate_name, Command};
use predicates::str::contains;

#[test]
pub fn cardano_install_works() -> Result<()> {
    let mut cmd = Command::cargo_bin(crate_name!())?;
    cmd.arg("install");
    cmd.assert().failure().stderr(contains("Install cardano components"));
    cmd.arg("help");
    cmd.assert().success().stdout(contains("Install cardano components"));
    Ok(())
}
