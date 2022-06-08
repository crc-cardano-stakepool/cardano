use crate::check_env;
use anyhow::Result;

pub async fn check_shell() -> Result<String> {
    check_env("SHELL")
}

#[cfg(test)]
mod test {
    // use crate::check_shell;
    #[tokio::test]
    #[ignore]
    async fn test_check_shell() {
        unimplemented!();
    }
}
