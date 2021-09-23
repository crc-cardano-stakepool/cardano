use super::color::print;
use super::dialog::proceed;
use super::env::*;
use anyhow::Result;
use console::Emoji;

pub async fn check_shell() -> Result<String> {
    let shell = check_env("SHELL")?;
    Ok(shell)
}

pub fn match_shell(shell: &str) -> Result<()> {
    let home = check_env("RUNNER_HOME")?;
    if shell.contains("/zsh") {
        print("green", "Found zsh", Emoji("", ""))?;
        let shell_profile_file = format!("{}/.zshrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "zsh");
    } else if shell.contains("/bash") {
        print("green", "Found bash", Emoji("", ""))?;
        let shell_profile_file = format!("{}/.bashrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "bash");
    } else if shell.contains("/sh") {
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

pub async fn setup_shell() -> Result<()> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    ask_shell_config().await?;
    Ok(())
}

pub async fn ask_shell_config() -> Result<()> {
    let shell = check_env("MY_SHELL")?;
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    if shell.is_empty() || shell_profile_file.is_empty() {
        panic!("No shell found")
    }
    let msg = format!("Detected {}", shell);
    print("green", &msg, Emoji("", ""))?;
    let msg = format!(
        "Do you want to automatically add the required PATH variables to {}",
        shell_profile_file
    );
    if proceed(&msg)? {
        let msg = format!(
            "Proceeding to add path variables for {} to {}",
            shell, shell_profile_file
        );
        print("magenta", &msg, Emoji("", ""))?;
    } else {
        print("red", "Skipped adding path variables", Emoji("", ""))?;
    }
    Ok(())
}
