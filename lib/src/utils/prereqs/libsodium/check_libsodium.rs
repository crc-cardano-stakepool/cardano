use crate::{async_command_pipe, install_libsodium, print};
use anyhow::Result;

pub async fn check_libsodium() -> Result<()> {
    print("", "Checking libsodium")?;
    let lib_files = &[
        "/usr/local/lib/libsodium.la ",
        "/usr/local/lib/libsodium.a ",
        "/usr/local/lib/libsodium.so",
    ];
    let expected = lib_files.concat();
    let cmd = format!("whereis libsodium | awk -F ':' {}", "'{print $2}'");
    let result = async_command_pipe(&cmd).await?;
    let result = result.trim();
    if result != expected {
        install_libsodium().await?;
    } else {
        print("green", "libsodium is installed")?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    // use crate::check_libsodium;
    #[tokio::test]
    #[ignore]
    async fn test_check_libsodium() {
        unimplemented!();
    }
}
