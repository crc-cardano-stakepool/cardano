#[cfg(test)]
pub mod test {
    use crate::cli::utils::types::TResult;
    use crate::cli::utils::Terminal;
    use assert_cmd::crate_name;
    use assert_cmd::Command;
    use console::Emoji;
    use predicates::str::contains;

    #[tokio::test]
    async fn async_command() {
        let res = Terminal::async_command("white", "file target/release/cardano", Emoji("", "")).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn cli_works() -> TResult<()> {
        let mut cmd = Command::cargo_bin(crate_name!())?;
        cmd.assert().failure().stderr(contains("Manage cardano components"));
        cmd.arg("help");
        cmd.assert().success().stdout(contains("Manage cardano components"));
        Ok(())
    }
}
