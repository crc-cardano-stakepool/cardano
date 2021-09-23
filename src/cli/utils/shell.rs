use super::color::print;
use super::config_map::PATH_CONFIG;
use super::dialog::proceed;
use super::env::*;
use super::process::{async_command, process_success};
use anyhow::Result;
use console::Emoji;

pub async fn setup_shell() -> Result<()> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    ask_shell_config().await?;
    Ok(())
}

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
        change_shell_config().await?;
    } else {
        print(
            "red",
            "Skipped adding path variables, setting at runtime manually",
            Emoji("", ""),
        )?;
    }
    Ok(())
}

pub async fn check_shell_config_env(pattern: &str) -> Result<bool> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let cmd = format!("grep -q {} {}", pattern, shell_profile_file);
    if process_success(&cmd).await? {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn change_shell_config() -> Result<()> {
    println!("Checking for shell configuration");
    for (key, value) in PATH_CONFIG.iter() {
        if let Ok(false) = check_shell_config_env(key).await {
            write_shell_config(value).await?;
        }
    }
    print("green", "Shell configured", Emoji("", ""))?;
    Ok(())
}

pub async fn write_shell_config(value: &str) -> Result<()> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let append_string = format!("$(cat << 'EOF'\n{}\nEOF\n)", value);
    let cmd = format!("echo \"{}\" >> {}", append_string, shell_profile_file);
    let msg = format!("Added line to {}: {}", shell_profile_file, value);
    print("green", &msg, Emoji("", ""))?;
    async_command(&cmd).await?;
    Ok(())
}
