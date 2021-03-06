use crate::{Environment, Executer, FileSystem, Git, CARDANO_URL};
use anyhow::Result;

pub fn update_cli() -> Result<()> {
    log::info!("Updating the CLI");
    FileSystem::setup_work_dir()?;
    let cardano_path = Environment::check_env("CARDANO_DIR")?;
    Git::check_repo(CARDANO_URL, &cardano_path)?;
    let cmd = format!(
        "
        cd {cardano_path} && 
        git checkout main && 
        git pull && 
        cargo install --path src/bin"
    );
    Executer::exec(&cmd)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    pub fn test_update_cli() -> Result<()> {
        update_cli()?;
        Ok(())
    }
}
