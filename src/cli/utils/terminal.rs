use anyhow::Result;
use console::Color;
use console::Emoji;
use console::Style;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm};
use std::process::Stdio;
use tokio::process::Command;

pub struct Terminal;

impl Terminal {
    pub async fn async_command(color: &str, command: &str, emoji: Emoji<'_, '_>) -> Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .output()
            .await;

        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Terminal::print(color, &stdout, emoji)
        } else {
            panic!("Error executing command: {}", command);
        }
    }

    pub fn print(color: &str, output: &str, emoji: Emoji<'_, '_>) -> Result<String> {
        match Terminal::to_color(&color) {
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
        Ok(String::from(output))
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
        let confirm = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(String::from(prompt))
            .interact()?;
        Ok(confirm)
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::utils::Terminal;
    use console::Emoji;

    #[tokio::test]
    pub async fn async_command() {
        let res = Terminal::async_command("white", "file target/release/cardano | awk '{print $2}'", Emoji("", "")).await;
        match res {
            Ok(res) => assert_eq!("ELF\n", res),
            Err(e) => panic!("{}", e),
        }
    }
}
