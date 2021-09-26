use crate::{check_env, print, set_env};
use anyhow::Result;

pub fn match_shell(shell: &str) -> Result<()> {
    let home = check_env("RUNNER_HOME")?;
    if shell.contains("/zsh") {
        print("green", "Found zsh")?;
        let shell_profile_file = format!("{}/.zshrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "zsh");
    } else if shell.contains("/bash") {
        print("green", "Found bash")?;
        let shell_profile_file = format!("{}/.bashrc", home);
        set_env("SHELL_PROFILE_FILE", &shell_profile_file);
        set_env("MY_SHELL", "bash");
    } else if shell.contains("/sh") {
        if !check_env("BASH")?.is_empty() {
            print("green", "Found bash")?;
            let shell_profile_file = format!("{}/.bashrc", home);
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "bash");
        } else if !check_env("ZSH_VERSION")?.is_empty() {
            print("green", "Found zsh")?;
            let shell_profile_file = format!("{}/.zshrc", home);
            set_env("SHELL_PROFILE_FILE", &shell_profile_file);
            set_env("MY_SHELL", "zsh");
        }
    } else {
        print("red", "No shell found, exporting variables manually")?;
    }
    Ok(())
}
