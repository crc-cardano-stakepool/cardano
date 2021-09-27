use crate::{async_command_pipe, set_env};
use anyhow::Result;

pub async fn check_user() -> Result<String> {
    let user = async_command_pipe("echo ${SUDO_USER:-$USER}").await?;
    set_env("RUNNER", &user);
    Ok(user)
}
