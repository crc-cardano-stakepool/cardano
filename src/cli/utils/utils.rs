use anyhow::Result;
use console::{Color, Emoji, Style, Term};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::path::Path;
use std::process::Stdio;
use sudo::{check, RunningAs};
use tokio::fs::create_dir_all;
use tokio::process::Command;

pub fn check_root() -> Result<bool> {
    if let RunningAs::Root = check() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn check_directory(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Checking for {} directory in {}", dir_name, absolute_path);
    print("", &msg, Emoji("", ""))?;
    if Path::new(absolute_path).is_dir() {
        let msg = format!("{} {}", dir_name, "directory found, skipped creating");
        print("green", &msg, Emoji("", ""))?;
    } else {
        create_directory(dir_name, absolute_path).await?;
    }
    Ok(())
}

pub async fn create_directory(dir_name: &str, absolute_path: &str) -> Result<()> {
    let msg = format!("Creating directory {} in {}", dir_name, absolute_path);
    print("", &msg, Emoji("", ""))?;
    create_dir_all(absolute_path).await?;
    Ok(())
}

pub async fn async_command(command: &str) -> Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .await?;
    Ok(String::from(String::from_utf8_lossy(&output.stdout)))
}

pub async fn check_install_dir() -> Result<String> {
    let items = vec!["~/.cardano", "Custom install directory"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the installation directory")
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;
    match selection {
        Some(index) => {
            let pick = items[index];
            println!("User selected item : {}", pick);
            match index {
                0 => Ok(String::from(pick)),
                _ => Ok(String::from("Custom install directory")),
            }
        }
        None => Ok(String::from("User did not select anything")),
    }
}

pub fn print(color: &str, output: &str, emoji: Emoji<'_, '_>) -> Result<()> {
    match to_color(&color) {
        Color::Cyan => {
            let cyan = format!("{} {}", Style::new().cyan().apply_to(output), emoji);
            Term::stdout().write_line(&cyan)?;
        }
        Color::Blue => {
            let blue = format!("{} {}", Style::new().blue().apply_to(output), emoji);
            Term::stdout().write_line(&blue)?;
        }
        Color::Black => {
            let black = format!("{} {}", Style::new().black().apply_to(output), emoji);
            Term::stdout().write_line(&black)?;
        }
        Color::Red => {
            let red = format!("{} {}", Style::new().red().apply_to(output), emoji);
            Term::stdout().write_line(&red)?;
        }
        Color::Green => {
            let green = format!("{} {}", Style::new().green().apply_to(output), emoji);
            Term::stdout().write_line(&green)?;
        }
        Color::Yellow => {
            let yellow = format!("{} {}", Style::new().yellow().apply_to(output), emoji);
            Term::stdout().write_line(&yellow)?;
        }
        Color::Magenta => {
            let magenta = format!("{} {}", Style::new().magenta().apply_to(output), emoji);
            Term::stdout().write_line(&magenta)?;
        }
        Color::White => {
            let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
            Term::stdout().write_line(&white)?;
        }
        _ => {
            let white = format!("{} {}", Style::new().white().apply_to(output), emoji);
            Term::stdout().write_line(&white)?;
        }
    };
    Ok(())
}

fn to_color(color: &str) -> Color {
    match color {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        _ => Color::White,
    }
}

pub fn proceed(prompt: &str) -> Result<bool> {
    let color_theme = &ColorfulTheme::default();
    let dialog = String::from(prompt);
    if Confirm::with_theme(color_theme).with_prompt(dialog).interact()? {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::utils::async_command;
    #[tokio::test]
    pub async fn test_async_command() {
        let res = async_command("file target/release/cardano | awk '{print $2}'").await;
        match res {
            Ok(res) => assert_eq!("ELF\n", res),
            Err(e) => panic!("{}", e),
        }
    }
}
