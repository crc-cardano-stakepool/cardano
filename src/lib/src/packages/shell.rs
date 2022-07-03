use crate::{async_command, async_command_pipe, check_env, proceed, process_success, set_env, setup_env, LD_LIBRARY_PATH, PKG_CONFIG_PATH};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

pub async fn ask_shell_config() -> Result<()> {
    let shell = check_env("MY_SHELL")?;
    let shell_file = check_env("SHELL_PROFILE_FILE")?;
    if shell.is_empty() || shell_file.is_empty() {
        return Err(anyhow!("No shell found"));
    }
    check_ask_shell_confirm(&shell_file).await
}

async fn check_ask_shell_confirm(shell_file: &str) -> Result<()> {
    let confirm = check_env("CONFIRM")?;
    let msg = format!("Do you want to automatically add the required PATH variables to {shell_file}");
    if confirm == "false" && proceed(&msg)? {
        return change_shell_config().await;
    }
    export_shell_variables().await
}

pub async fn change_shell_config() -> Result<()> {
    let paths = HashMap::from([
        ("LD_LIBRARY_PATH", "export LD_LIBRARY_PATH=\"/usr/local/lib:$LD_LIBRARY_PATH\""),
        (
            "PKG_CONFIG_PATH",
            "export PKG_CONFIG_PATH=\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\"",
        ),
        (
            "CARDANO_NODE_SOCKET_PATH",
            "export CARDANO_NODE_SOCKET_PATH=\"$HOME/.cardano/ipc/node.socket\"",
        ),
        (".local/bin", "export PATH=\"$HOME/.local/bin:$PATH\""),
        (".cabal/bin", "export PATH=\"$HOME/.cabal/bin:$PATH\""),
        (".ghcup/bin", "export PATH=\"$HOME/.ghcup/bin:$PATH\""),
    ]);
    for (key, value) in paths.iter() {
        if !check_shell_config_env(key).await? {
            write_shell_config(value).await?;
        }
    }
    Ok(())
}

pub async fn check_shell_config_env(pattern: &str) -> Result<bool> {
    log::debug!("Checking if shell profile is already configured");
    let shell_profile_file = get_shell_profile_file().await?;
    let cmd = format!("grep -q {pattern} {shell_profile_file}");
    process_success(&cmd).await
}

pub fn check_shell() -> String {
    match check_env("SHELL") {
        Ok(shell) => shell,
        Err(_) => "/usr/bin/bash".to_string(),
    }
}

pub async fn export_shell_variables() -> Result<()> {
    log::debug!("Exporting shell variables");
    let envs = HashMap::from([("LD_LIBRARY_PATH", LD_LIBRARY_PATH), ("PKG_CONFIG_PATH", PKG_CONFIG_PATH)]);
    for (key, value) in envs.iter() {
        set_env(key, value);
    }
    source_shell().await
}

pub async fn get_shell_profile_file() -> Result<String> {
    log::debug!("Getting shell profile");
    match_shell(&check_shell())?;
    check_env("SHELL_PROFILE_FILE")
}

pub fn match_shell(shell: &str) -> Result<()> {
    log::debug!("Matching shell");
    let home = check_env("HOME")?;
    if shell.contains("/zsh") {
        let shell_profile_file = format!("{home}/.zshrc");
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "zsh");
        return Ok(());
    }
    if shell.contains("/bash") {
        let shell_profile_file = format!("{home}/.bashrc");
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "bash");
        return Ok(());
    }
    if shell.contains("/sh") {
        if !check_env("BASH")?.is_empty() {
            let shell_profile_file = format!("{home}/.bashrc");
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "bash");
            return Ok(());
        }
        if !check_env("ZSH_VERSION")?.is_empty() {
            let shell_profile_file = format!("{home}/.zshrc");
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "zsh");
            return Ok(());
        }
        return Err(anyhow!("Failed checking shell"));
    }
    Ok(())
}

pub async fn setup_shell() -> Result<()> {
    log::info!("Setting up shell");
    let shell = check_shell();
    match_shell(&shell)?;
    ask_shell_config().await?;
    setup_env()
}

pub async fn source_shell() -> Result<()> {
    log::debug!("Sourcing shell");
    let shell_file = get_shell_profile_file().await?;
    let cmd = format!("source {shell_file}");
    async_command_pipe(&cmd).await?;
    Ok(())
}

pub async fn write_shell_config(value: &str) -> Result<()> {
    let shell_profile_file = check_env("SHELL_PROFILE_FILE")?;
    log::info!("Writing {value} to {shell_profile_file}");
    let append_string = format!("$(cat << 'EOF'\n{value}\nEOF\n)");
    let cmd = format!("echo \"{append_string}\" >> {shell_profile_file}");
    async_command(&cmd).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;

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
