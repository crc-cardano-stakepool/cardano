use crate::{
    async_command, async_command_pipe, check_env, print, proceed, process_success, set_env, setup_env, ENVS, PATHS,
};
use anyhow::{anyhow, Result};

pub async fn ask_shell_config() -> Result<()> {
    let shell = check_env("MY_SHELL")?;
    let shell_file = check_env("SHELL_PROFILE_FILE")?;
    if shell.is_empty() || shell_file.is_empty() {
        return Err(anyhow!("No shell found"));
    }
    let msg = format!("Detected {}", shell);
    print("green", &msg)?;
    check_ask_shell_confirm(&shell, &shell_file).await
}

async fn check_ask_shell_confirm(shell: &str, shell_file: &str) -> Result<()> {
    let confirm = check_env("CONFIRM")?;
    let msg = format!(
        "Do you want to automatically add the required PATH variables to {}",
        shell_file
    );
    if confirm == "false" && proceed(&msg)? {
        let msg = format!("Proceeding to add path variables for {} to {}", shell, shell_file);
        print("magenta", &msg)?;
        change_shell_config().await
    } else {
        print("yellow", "Skipped adding path variables")?;
        export_shell_variables().await
    }
}

pub async fn change_shell_config() -> Result<()> {
    print("", "Checking for shell configuration")?;
    for (key, value) in PATHS.iter() {
        if !check_shell_config_env(key).await? {
            write_shell_config(value).await?;
        }
    }
    print("green", "Shell configured")
}

pub async fn check_shell_config_env(pattern: &str) -> Result<bool> {
    let shell_profile_file = get_shell_profile_file().await?;
    let cmd = format!("grep -q {} {}", pattern, shell_profile_file);
    process_success(&cmd).await
}

pub async fn check_shell() -> Result<String> {
    check_env("SHELL")
}

pub async fn export_shell_variables() -> Result<()> {
    print("", "Exporting shell variables")?;
    for (key, value) in ENVS.iter() {
        set_env(key, value);
    }
    source_shell().await
}

pub async fn get_shell_profile_file() -> Result<String> {
    match_shell(&check_shell().await?)?;
    check_env("SHELL_PROFILE_FILE")
}

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

pub async fn setup_shell() -> Result<()> {
    let shell = check_shell().await?;
    match_shell(&shell)?;
    ask_shell_config().await?;
    setup_env().await
}

pub async fn source_shell() -> Result<()> {
    let shell_file = get_shell_profile_file().await?;
    let cmd = format!("source {}", shell_file);
    async_command_pipe(&cmd).await?;
    print("green", "Sourced shell")
}

pub async fn write_shell_config(value: &str) -> Result<()> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    let append_string = format!("$(cat << 'EOF'\n{}\nEOF\n)", value);
    let cmd = format!("echo \"{}\" >> {}", append_string, shell_profile_file);
    let msg = format!("Added line to {}: {}", shell_profile_file, value);
    async_command(&cmd).await?;
    print("green", &msg)
}

#[cfg(test)]
mod test {
    // use super::*;
    //
    #[tokio::test]
    #[ignore]
    async fn test_write_shell_config() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_setup_shell() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_match_shell() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_change_shell_config() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_shell() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_check_shell_config_env() {
        unimplemented!();
    }

    #[tokio::test]
    #[ignore]
    async fn test_ask_shell_config() {
        unimplemented!();
    }
}
