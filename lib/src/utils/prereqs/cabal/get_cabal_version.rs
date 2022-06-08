pub fn get_cabal_version_url() -> &'static str {
    if let Some(url) = URLS.get("versions") {
        url
    } else {
        "https://developers.cardano.org/docs/get-started/installing-cardano-node"
    }
}

use crate::async_command_pipe;
use crate::URLS;
use anyhow::Result;

pub async fn get_cabal_version() -> Result<String> {
    let cmd = format!(
        "{} {} | {} | {} | {} | {} | {}",
        "curl -s",
        get_cabal_version_url(),
        "fold -w100",
        "grep '<code>cabal '",
        "sed 's/^.*cabal //'",
        "awk -F '<' '{print $1}'",
        "tail -n1"
    );
    let cabal_version = async_command_pipe(&cmd).await?;
    let cabal_version = cabal_version.trim();
    println!("{cabal_version}");
    Ok(String::from(cabal_version))
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_get_cabal_version() -> Result<()> {
        let version = get_cabal_version().await?;
        assert_eq!(version, "3.6.2.0");
        Ok(())
    }
}
