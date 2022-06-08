use crate::{async_command_pipe, set_env};
use sudo::{check, RunningAs};
use anyhow::Result;

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
    // use super::*;
    #[tokio::test]
    #[ignore]
    async fn test_check_user() {
        unimplemented!();
    }
    #[tokio::test]
    #[ignore]
    async fn test_check_root() {
        unimplemented!();
    }
}
