use crate::{absolute_ref_path_to_string, async_command_pipe, check_env, process_success, read_setting, set_env, setup_env};
use anyhow::{anyhow, Result};
use std::{io::Write, path::PathBuf};

#[derive(Debug, Copy, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

pub struct ShellConfig {
    pub shell: Shell,
    pub config_file: PathBuf,
}

impl Default for ShellConfig {
    fn default() -> Self {
        let shell = Self::check_shell();
        let shell = Self::match_shell(&shell);
        let config_file = Self::match_config_file(shell);
        let parsed_config_file = absolute_ref_path_to_string(&config_file).unwrap();
        set_env("SHELL_CONFIG_FILE", &parsed_config_file);
        Self { shell, config_file }
    }
}

impl ShellConfig {
    pub fn check_shell() -> String {
        match check_env("SHELL") {
            Ok(shell) => shell,
            Err(_) => "/usr/bin/bash".to_string(),
        }
    }
    pub fn match_shell(shell: &str) -> Shell {
        if shell.contains("/bash") {
            return Shell::Bash;
        }
        if shell.contains("/zsh") {
            return Shell::Zsh;
        }
        if shell.contains("/sh") {
            if !check_env("BASH")
                .map_err(|err| anyhow!("Failed to read $BASH environment variable: {err}"))
                .unwrap()
                .is_empty()
            {
                return Shell::Bash;
            }
            if !check_env("ZSH_VERSION")
                .map_err(|err| anyhow!("Failed to read $ZSH_VERSION environment variable: {err}"))
                .unwrap()
                .is_empty()
            {
                return Shell::Zsh;
            }
        }
        Shell::Bash
    }

    pub fn match_config_file(shell: Shell) -> PathBuf {
        match shell {
            Shell::Bash => {
                let mut config = dirs::home_dir().expect("Read $HOME");
                config.push(".bashrc");
                config
            }
            Shell::Zsh => {
                let mut config = dirs::home_dir().expect("Read $HOME");
                config.push(".zshrc");
                config
            }
        }
    }

    pub async fn setup_shell() -> Result<()> {
        log::info!("Setting up shell");
        let shell = ShellConfig::default();
        shell.change_shell_config().await?;
        setup_env()
    }

    pub async fn change_shell_config(&self) -> Result<()> {
        let patterns = vec!["LD_LIBRARY_PATH", "PKG_CONFIG_PATH", ".local/bin", ".cabal/bin", ".ghcup/bin"];
        let paths = vec![
            "export LD_LIBRARY_PATH=\"/usr/local/lib:$LD_LIBRARY_PATH\"",
            "export PKG_CONFIG_PATH=\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\"",
            "export PATH=\"$HOME/.local/bin:$PATH\"",
            "export PATH=\"$HOME/.cabal/bin:$PATH\"",
            "export PATH=\"$HOME/.ghcup/bin:$PATH\"",
        ];
        self.write_node_socket_path().await?;
        for (pattern, path) in patterns.iter().zip(paths.iter()) {
            if !self.check_shell_config_env(pattern).await? {
                self.write_shell_config(path);
            }
        }
        Ok(())
    }

    pub async fn check_shell_config_env(&self, pattern: &str) -> Result<bool> {
        log::debug!("Checking shell configuration");
        let config_file = absolute_ref_path_to_string(&self.config_file).unwrap();
        let cmd = format!("grep -q {pattern} {config_file}");
        process_success(&cmd).await
    }

    pub fn write_shell_config(&self, value: &str) {
        let config_file = absolute_ref_path_to_string(&self.config_file).unwrap();
        log::info!("Writing {value} to {config_file}");
        let mut f = std::fs::File::options()
            .write(true)
            .append(true)
            .open(&self.config_file)
            .map_err(|err| anyhow!("Failed to open {config_file}: {err}"))
            .unwrap();
        writeln!(f, "{value}")
            .map_err(|err| anyhow!("Failed to write {value} to {config_file}: {err}"))
            .unwrap();
    }

    pub async fn write_node_socket_path(&self) -> Result<()> {
        let node_socket_path = read_setting("node_socket_path")?;
        let value = format!("export CARDANO_NODE_SOCKET_PATH={node_socket_path}");
        if !self.check_shell_config_env("CARDANO_NODE_SOCKET_PATH").await? {
            self.write_shell_config(&value);
        }
        Ok(())
    }

    pub async fn source_shell() -> Result<()> {
        log::debug!("Sourcing shell");
        let config_file = check_env("SHELL_CONFIG_FILE")?;
        let cmd = format!("source {}", config_file);
        async_command_pipe(&cmd).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::ShellConfig;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_write_shell_config() {
        unimplemented!();
    }

    #[tokio::test]
    async fn test_write_note_socket_path() -> Result<()> {
        let shell = ShellConfig::default();
        shell.write_node_socket_path().await?;
        Ok(())
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
