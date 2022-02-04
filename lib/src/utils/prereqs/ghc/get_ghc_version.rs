use crate::{async_command_pipe, get_ghc_version_url};
use anyhow::Result;
pub async fn get_ghc_version() -> Result<String> {
    let cmd = format!(
        "{} {} | {} | {} | {} | {} | {}",
        "curl -s",
        get_ghc_version_url(),
        "fold -w100",
        "grep '<code>ghc '",
        "sed 's/^.*ghc //'",
        "awk -F '<' '{print $1}'",
        "tail -n1"
    );
    let ghc_version = async_command_pipe(&cmd).await?;
    let ghc_version = ghc_version.trim();
    println!("{ghc_version}");
    Ok(String::from(ghc_version))
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_get_ghc_version() -> Result<()> {
        let version = get_ghc_version().await?;
        assert_eq!(version, "8.10.7");
        Ok(())
    }
}
