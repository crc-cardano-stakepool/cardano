use crate::{check_env, set_env};
use anyhow::Result;

pub fn check_user() -> Result<String> {
    let user = match check_env("SUDO_USER") {
        Ok(sudo_user) => sudo_user,
        Err(_) => check_env("USER").unwrap(),
    };
    log::debug!("user: {user}");
    set_env("RUNNER", &user);
    Ok(user.trim().to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::check_env;

    #[tokio::test]
    async fn test_check_user() -> Result<()> {
        let user = check_user()?;
        log::debug!("user: {user}");
        let user_env = check_env("RUNNER")?;
        log::debug!("user_env: {user_env}");
        assert_eq!(user, user_env);
        Ok(())
    }
}
