use crate::{check_user, set_env};
use anyhow::Result;

pub async fn check_home_dir() -> Result<String> {
    let user = check_user().await?;
    let home_directory = format!("/home/{}", user.trim());
    set_env("RUNNER_HOME", &home_directory);
    Ok(home_directory)
}
