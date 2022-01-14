use crate::{async_command_pipe, get_ghc_version_url};
use anyhow::Result;
pub async fn get_ghc_version() -> Result<String> {
    let cmd = format!(
        "{} {} | {} | {} | {} | {} | {}",
        "curl -s",
        get_ghc_version_url(),
        "fold -w100",
        "grep '<code>ghc '",
        "awk '{print $3}'",
        "awk -F '<' '{print $1}'",
        "tail -n1"
    );
    let ghc_version = async_command_pipe(&cmd).await?;
    Ok(String::from(ghc_version.trim()))
}

#[cfg(test)]
mod test {
    // use crate::get_ghc_version;
    #[test]
    #[ignore]
    fn test_get_ghc_version() {
        unimplemented!();
    }
}
