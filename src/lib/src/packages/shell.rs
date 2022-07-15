use crate::{Environment, Executer, FileSystem, Settings};
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
        let parsed_config_file =
            FileSystem::absolute_ref_path_to_string(&config_file).unwrap();
        Environment::set_env("SHELL_CONFIG_FILE", &parsed_config_file);
        Self { shell, config_file }
    }
}

impl ShellConfig {
    pub fn check_shell() -> String {
        Environment::check_env("SHELL")
            .map_or_else(|_| "/usr/bin/bash".to_string(), |value| value)
    }

    pub fn match_shell(shell: &str) -> Shell {
        if shell.contains("/bash") {
            return Shell::Bash;
        }
        if shell.contains("/zsh") {
            return Shell::Zsh;
        }
        if shell.contains("/sh") {
            if !Environment::check_env("BASH")
                .map_err(|err| {
                    anyhow!("Failed to read $BASH environment variable: {err}")
                })
                .unwrap()
                .is_empty()
            {
                return Shell::Bash;
            }
            if !Environment::check_env("ZSH_VERSION")
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
                if config.exists() {
                    return config;
                }
                Environment::check_env("ZDOTDIR")
                    .map(|path| {
                        let mut path = PathBuf::from(&path);
                        path.push(".zshrc");
                        if !path.exists() {
                            panic!("Could not find .zshrc")
                        }
                        path
                    })
                    .map_err(|err| anyhow!("Failed to read ZDOTDIR: {err}"))
                    .unwrap()
            }
        }
    }

    pub fn setup_shell() -> Result<()> {
        log::info!("Setting up shell");
        let shell = ShellConfig::default();
        shell.change_shell_config()?;
        Environment::setup_env()
    }

    pub fn change_shell_config(&self) -> Result<()> {
        let patterns = vec![
            "LD_LIBRARY_PATH",
            "PKG_CONFIG_PATH",
            ".local/bin",
            ".cabal/bin",
            ".ghcup/bin",
        ];
        let paths = vec![
            "export LD_LIBRARY_PATH=\"/usr/local/lib:$LD_LIBRARY_PATH\"",
            "export PKG_CONFIG_PATH=\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\"",
            "export PATH=\"$HOME/.local/bin:$PATH\"",
            "export PATH=\"$HOME/.cabal/bin:$PATH\"",
            "export PATH=\"$HOME/.ghcup/bin:$PATH\"",
        ];
        self.write_node_socket_path()?;
        for (pattern, path) in patterns.iter().zip(paths.iter()) {
            if !self.check_shell_config_env(pattern)? {
                self.write_shell_config(path);
            }
        }
        Executer::exec(
            "export LD_LIBRARY_PATH=\"/usr/local/lib:$LD_LIBRARY_PATH\"",
        )?;
        Executer::exec("export PKG_CONFIG_PATH=\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\"")?;
        Ok(())
    }

    pub fn check_shell_config_env(&self, pattern: &str) -> Result<bool> {
        log::debug!("Checking shell configuration");
        let config_file =
            FileSystem::absolute_ref_path_to_string(&self.config_file).unwrap();
        let cmd = format!("grep -q {pattern} {config_file}");
        Executer::success(&cmd)
    }

    pub fn write_shell_config(&self, value: &str) {
        let config_file =
            FileSystem::absolute_ref_path_to_string(&self.config_file).unwrap();
        log::info!("Writing {value} to {config_file}");
        let mut f = std::fs::File::options()
            .write(true)
            .append(true)
            .open(&self.config_file)
            .map_err(|err| anyhow!("Failed to open {config_file}: {err}"))
            .unwrap();
        writeln!(f, "{value}")
            .map_err(|err| {
                anyhow!("Failed to write {value} to {config_file}: {err}")
            })
            .unwrap();
    }

    pub fn write_node_socket_path(&self) -> Result<()> {
        let node_socket_path = Settings::read("node_socket_path")?;
        let value =
            format!("export CARDANO_NODE_SOCKET_PATH={node_socket_path}");
        if !self.check_shell_config_env("CARDANO_NODE_SOCKET_PATH")? {
            self.write_shell_config(&value);
        }
        Ok(())
    }

    pub fn source_shell() -> Result<()> {
        log::debug!("Sourcing shell");
        let config_file = Environment::check_env("SHELL_CONFIG_FILE")?;
        let cmd = format!("source {}", config_file);
        Executer::exec(
            "export LD_LIBRARY_PATH=\"/usr/local/lib:$LD_LIBRARY_PATH\"",
        )?;
        Executer::exec("export PKG_CONFIG_PATH=\"/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH\"")?;
        Executer::capture(&cmd)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ShellConfig;

    #[test]
    fn test_write_note_socket_path() -> Result<()> {
        let shell = ShellConfig::default();
        shell.write_node_socket_path()?;
        Ok(())
    }
}
