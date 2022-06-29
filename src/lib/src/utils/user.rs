use crate::set_env;
use anyhow::Result;
use nix::unistd::{getuid, User};

pub fn check_user() -> Result<String> {
    let uid = getuid();
    let res = User::from_uid(uid).unwrap().unwrap();
    let user = res.name;
    set_env("RUNNER", &user);
    Ok(user)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::check_env;

    #[tokio::test]
    async fn test_check_user() -> Result<()> {
        let user = check_user()?;
        let user_env = check_env("RUNNER")?;
        assert_eq!(user, user_env);
        Ok(())
    }
}
