use crate::{async_command_pipe, set_env};
use anyhow::Result;
use sudo::{check, RunningAs};

pub fn check_root() -> Result<bool> {
    if let RunningAs::Root = check() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn check_user() -> Result<String> {
    let user = async_command_pipe("echo ${SUDO_USER:-$USER}").await?;
    let user = user.trim();
    set_env("RUNNER", user);
    Ok(user.to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::check_env;

    #[tokio::test]
    async fn test_check_root() -> Result<()> {
        let root = check_root()?;
        assert!(!root);
        Ok(())
    }

    #[tokio::test]
    async fn test_check_user() -> Result<()> {
        let user = check_user().await?;
        let user_env = check_env("RUNNER")?;
        assert_eq!(user, user_env);
        Ok(())
    }
}
