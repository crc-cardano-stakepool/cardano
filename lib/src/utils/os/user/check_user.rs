use crate::{async_command_pipe, set_env};
use anyhow::Result;

pub async fn check_user() -> Result<String> {
    let user = async_command_pipe("echo ${SUDO_USER:-$USER}").await?;
    set_env("RUNNER", &user);
    Ok(user)
}

#[cfg(test)]
mod test {
    // use crate::check_user;
    #[tokio::test]
    #[ignore]
    async fn test_check_user() {
        unimplemented!();
    }
}
