use super::color::print;
use super::env::*;
use anyhow::Result;
use console::Emoji;

pub async fn check_shell() -> Result<String> {
    let shell = check_env("SHELL")?;
    Ok(shell)
}

pub async fn setup_shell() -> Result<()> {
    let home = check_env("RUNNER_HOME")?;
    let shell = check_shell().await?;
    if shell.as_str().contains("/zsh") {
        print("green", "Found zsh", Emoji("", ""))?;
        let shell_profile_file = format!("{}/.zshrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "zsh");
    } else if shell.as_str().contains("/bash") {
        print("green", "Found bash", Emoji("", ""))?;
        let shell_profile_file = format!("{}/.bashrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "bash");
    } else if shell.as_str().contains("/sh") {
        if !check_env("BASH")?.is_empty() {
            print("green", "Found bash", Emoji("", ""))?;
            let shell_profile_file = format!("{}/.bashrc", home);
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "bash");
        } else if !check_env("ZSH_VERSION")?.is_empty() {
            print("green", "Found zsh", Emoji("", ""))?;
            let shell_profile_file = format!("{}/.zshrc", home);
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "zsh");
        }
    } else {
        print("red", "No shell found, exporting variables manually", Emoji("", ""))?;
    }
    Ok(())
}
