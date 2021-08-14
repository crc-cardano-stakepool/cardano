use console::Emoji;
use lib::cli::utils::Terminal;

#[tokio::test]
pub async fn async_command() {
    let res = Terminal::async_command("white", "file target/release/cardano >/dev/null", Emoji("", "")).await;
    assert!(res.is_ok());
}
