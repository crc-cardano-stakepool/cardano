use crate::{check_env, print, set_env};
use anyhow::{anyhow, Result};

pub fn match_shell(shell: &str) -> Result<()> {
    let home = check_env("RUNNER_HOME")?;
    if shell.contains("/zsh") {
        let shell_profile_file = format!("{}/.zshrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "zsh");
        Ok(())
    } else if shell.contains("/bash") {
        let shell_profile_file = format!("{}/.bashrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "bash");
        Ok(())
    } else if shell.contains("/sh") {
        if !check_env("BASH")?.is_empty() {
            let shell_profile_file = format!("{}/.bashrc", home);
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "bash");
            print("green", "Detected bash")
        } else if !check_env("ZSH_VERSION")?.is_empty() {
            let shell_profile_file = format!("{}/.zshrc", home);
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "zsh");
            print("green", "Detected zsh")
        } else {
            Err(anyhow!("Failed checking shell"))
        }
    } else {
        print("red", "No shell found, exporting variables manually")
    }
}

#[cfg(test)]
mod test {
    // use crate::match_shell;
    #[tokio::test]
    #[ignore]
    async fn test_match_shell() {
        unimplemented!();
    }
}
