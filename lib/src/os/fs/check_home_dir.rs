use crate::{check_user, set_env};
use anyhow::Result;

pub async fn check_home_dir() -> Result<String> {
    let user = check_user().await?;
    let home_directory = format!("/home/{}", user.trim());
    set_env("RUNNER_HOME", &home_directory);
    Ok(home_directory)
}

#[cfg(test)]
mod test {
    // use crate::check_home_dir;
    #[tokio::test]
    #[ignore]
    async fn test_check_home_dir() {
        unimplemented!();
    }
}
