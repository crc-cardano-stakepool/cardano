use crate::process_success;
use anyhow::Result;

pub async fn is_program_installed(program: &str) -> Result<bool> {
    let cmd = format!("type {}", program);
    process_success(&cmd).await
}

#[cfg(test)]
mod test {
    // use crate::install_ghc;
    #[tokio::test]
    #[ignore]
    async fn test_install_ghc() {
        unimplemented!();
    }
}
